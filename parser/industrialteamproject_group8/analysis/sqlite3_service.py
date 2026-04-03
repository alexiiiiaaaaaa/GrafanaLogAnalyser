# class that's responsible for handling the database by listening for changes, querying the database and deleting entries
# now in SQLite3

import sqlite3, time, json, threading, os
from datetime import datetime
from .handler_generic import GenericHandler

class SQLite3Service:
    def __init__(self, sqlFilepath, configFilepath):
        self.connection = sqlite3.connect(sqlFilepath, check_same_thread=False)
        self.connection.row_factory = sqlite3.Row
        self.cursor = self.connection.cursor()

        self.alertQueue = []
        self.tables = []
        self.configFilepath = configFilepath
        self._listener_thread = None
        self._running = False
        self._last_seen_ts = 0

        self.readTables()

    # gets a list of table names from the config file
    def readTables(self):
        with open(self.configFilepath) as file:
            self.tables = [line.rstrip() for line in file]

    # run an SQL query and it'll return a list of cursors pointing to relavent documents
    # this is for non-listener reads
    def execute(self, query, params=()):
        return self.cursor.execute(query, params).fetchall()
    
    # waits for a change to the database, sends any change to handle_change()
    def _listener(self):
        while self._running:
            cursor = self.connection.cursor()

            rows = []
            for table in self.tables:
                result = cursor.execute(
                    f"SELECT * FROM {table} WHERE timestamp > ? ORDER BY timestamp ASC",
                    (self._last_seen_ts,)
                ).fetchall()

                rows.extend({**dict(row), "table": table} for row in result)

            if rows:
                rows = self.timestamp_fixer(rows)
                rows.sort(key=lambda r: r["timestamp"])
                for row in rows:
                    self.handle_change(row)

                if self.alertQueue:
                    self.append_changes()

                # update last seen timestamp
                self._last_seen_ts = rows[-1]["timestamp"].isoformat() + "Z"
            time.sleep(4)

    # takes the change from the listener and sends it to the factory
    def handle_change(self, row):
        processor = GenericHandler.create(self, row)
        for alert in processor.identify():
            self.alertQueue.append(alert)

    # start the listener on its own thread
    def start_listener(self):
        if self._listener_thread and self._listener_thread.is_alive():
            return

        self._running = True
        self._listener_thread = threading.Thread(
            target=self._listener,
            daemon=True
        )
        self._listener_thread.start()

    # append changes to the jsonl file
    def append_changes(self):
        file_path = os.path.join("output", "anomalies.jsonl")

        self.alertQueue = self.timestamp_unfixer(self.alertQueue)

        with open(file_path, 'a', encoding='utf-8') as f:
            for record in self.alertQueue:
                json_line = json.dumps(record)
                f.write(json_line + '\n')

        self.alertQueue.clear()

    # fix these stringy timestamp fields
    def timestamp_fixer(self, rows):
        for row in rows:
            if row["timestamp"].endswith("Z"):
                row["timestamp"] = row["timestamp"][:-1]
            
            row["timestamp"] = datetime.fromisoformat(row["timestamp"])
        
        return rows

    # stringify timestamps
    def timestamp_unfixer(self, rows):
        fixed = []
        for row in rows:
            ts = row.get("timestamp")

            if hasattr(ts, "isoformat"):
                row["timestamp"] = ts.isoformat() + "Z"

            fixed.append((row))

        return fixed

    # stop the listener
    def stop_listener(self):
        self._running = False
        if self._listener_thread:
            self._listener_thread.join()

    # make sure the listener's stopped if the object's deleted
    def close(self):
        self.stop_listener()
        self.connection.close()