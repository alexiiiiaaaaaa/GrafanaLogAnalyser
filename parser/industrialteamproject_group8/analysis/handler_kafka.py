# subclass responsible for checking for anomalies which are relevant for kafka logs: A2 and A7
from .handler_generic import GenericHandler
from datetime import datetime

class KafkaHandler(GenericHandler):
    table = "kafka_atm_metrics"

    def identify(self):
        alerts = []

        if self.anomaly_2():
            self.row.update({"detected_anomaly": "A2: Cash Cassettes Empty"})
            alerts.append(self.row)

        if self.anomaly_7():
            self.row.update({"detected_anomaly": self.anomaly_7()})
            alerts.append(self.row)

        return alerts
    
    def anomaly_2(self):
        # if the kafka log isn't a CASH_DISPENSE_ERROR or transaction_success_rate=0.0 then this isn't an A2
        if self.row["transaction_failure_reason"] != "CASH_DISPENSE_ERROR" or self.row["transaction_success_rate"] != 0:
            return None
      
        # check for a recent CASH_DISPENSE_ERROR
        query = """
            SELECT * FROM kafka_atm_metrics
            WHERE atm_id = ?
            AND atm_status = ?
            AND transaction_failure_reason = ?
            AND timestamp >= ?
            AND timestamp < ?
        """
        params = (
            self.row["atm_id"],
            "Out of Service",
            "CASH_DISPENSE_ERROR",
            self.time_window_back(1800),
            self.time_window_forward(30)
        )

        if not self.database.execute(query, params):
            return None
        
        # check for a recent transaction success rate 0
        query = """
            SELECT * FROM kafka_atm_metrics
            WHERE atm_id = ?
            AND transaction_rate_tps = ?
            AND transaction_success_rate = ?
            AND timestamp >= ?
            AND timestamp < ?
        """
        params = (
            self.row["atm_id"],
            0,
            0,
            self.time_window_back(1800),
            self.time_window_forward(30)
        )

        if not self.database.execute(query, params):
            return None
        
        # check for recent CASSETTE_EMPTYs
        query = """
            SELECT * FROM hardware_sensor_log
            WHERE atm_id = ?
            AND event_type = ?
            AND timestamp >= ?
            AND timestamp < ?
        """
        params = (
            self.row["atm_id"],
            "CASSETTE_EMPTY",
            self.time_window_back(1800),
            self.time_window_forward(30)
        )

        if not self.database.execute(query, params):
            return None
        
        # worry about CASSETTE_LOWs later

        return True
    
    def anomaly_7(self):
        alerts = ""
        # check for missing fields
        if self.row["atm_status"] == None or self.row["transaction_rate_tps"] == None:
            alerts = alerts + "A7: Schema validation failure"
        
        # check for out of order sequence
        query = """
            SELECT * FROM kafka_atm_metrics
            WHERE id = ?
            AND timestamp = ?
        """
        params = (
            self.row["id"] - 1,
            self.row["timestamp"].isoformat()
        )

        row = self.database.execute(query, params)

        if row:
            if row[0]["event_id"][7] > self.row["event_id"][7]:
                alerts = alerts + "A7: Out of order sequence"

        return alerts