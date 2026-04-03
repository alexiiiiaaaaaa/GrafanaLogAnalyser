# subclass responsible for checking for anomalies which are relevant for terminal handler logs: A1, A3 and A4
from .handler_generic import GenericHandler

class TerminalHandler(GenericHandler):
    table = "terminal_handler_log"

    def identify(self):
        alerts = []

        if self.anomaly_1():
            self.row.update({"detected_anomaly": "A1: Network Timeout Cascade"})
            alerts.append(self.row)

        if self.anomaly_3():
            self.row.update({"detected_anomaly": "A3: JVM Memory Leak"})
            alerts.append(self.row)

        if self.anomaly_4():
            self.row.update({"detected_anomaly": "A4: Container Restart Loop"})
            alerts.append(self.row)

        return alerts
    
    def anomaly_1(self):
        # if the terminal handler log isn't a network timeout then this isn't an A1
        if self.row["event_type"] != "NETWORK_TIMEOUT":
            return None

        # check for a kafka log
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
            "Offline",
            "HOST_UNAVAILABLE",
            self.time_window_back(60),
            self.time_window_forward(60)
        )

        if not self.database.execute(query, params):
            return None
        
        # check for a TIMEOUT atm app log
        query = """
            SELECT * FROM atm_application_log
            WHERE atm_id = ?
            AND event_type = ?
            AND response_time_ms >= ?
            AND timestamp >= ?
            AND timestamp < ?
        """
        params = (
            self.row["atm_id"],
            "TIMEOUT",
            10000,
            self.time_window_back(60),
            self.time_window_forward(60)
        )

        if not self.database.execute(query, params):
            return None
        
        # check for a NETWORK_DISCONNECT atm app log
        query = """
            SELECT * FROM atm_application_log
            WHERE atm_id = ?
            AND event_type = ?
            AND error_code = ?
            AND timestamp >= ?
            AND timestamp < ?
        """
        params = (
            self.row["atm_id"],
            "NETWORK_DISCONNECT",
            "ERR-0040",
            self.time_window_back(60),
            self.time_window_forward(60)
        )

        if not self.database.execute(query, params):
            return None

        return True

    def anomaly_3(self):
        GCP_CPU_THRESHOLD = 0.4
        PROMETHEUS_GC_PAUSE_THRESHOLD = 10
        PROMETHEUS_CPU_USAGE_THRESHOLD = 0.7

        # if the terminal handler log isn't a FATAL OutOfMemoryError then this isn't an A3
        if self.row["exception_class"] != "java.lang.OutOfMemoryError" or self.row["log_level"] != "FATAL":
            return None
        
        # check high gcp cpu usage
        query = """
            SELECT * FROM gcp_cloud_metrics
            WHERE metric_name = ?
            AND timestamp < ?
            ORDER BY timestamp DESC
            LIMIT 1
        """
        params = (
            "container/cpu/usage_time",
            self.time_window_forward(10)
        )

        row =  self.database.execute(query, params)

        if row:
            if row[0]["metric_value"] < GCP_CPU_THRESHOLD:
                return None
                 
        # check for monotonic increase in container/cpu/usage_time
        query = """
            SELECT * FROM (
                SELECT *,
                    LAG(metric_value) OVER (ORDER BY timestamp) AS prev_metric_value
                FROM gcp_cloud_metrics
            ) AS sub
            WHERE metric_value <= prev_metric_value
            AND metric_name = ?
            AND timestamp >= ?
            AND timestamp < ?
        """
        params = (
            "container/cpu/usage_time",
            self.time_window_back(180),
            self.time_window_forward(180)
        )

        if self.database.execute(query, params):
            return None

        # check for prometheus jvm_gc_pause_seconds_sum threshold
        query = """
            SELECT * FROM prometheus_metrics
            WHERE metric_name = ?
            AND timestamp >= ?
            AND timestamp < ?
            ORDER BY timestamp DESC
            LIMIT 1
        """

        params = (
            "jvm_gc_pause_seconds_sum",
            self.time_window_back(180),
            self.time_window_forward(180)
        )

        if row:
            if row[0]["metric_value"] < PROMETHEUS_GC_PAUSE_THRESHOLD:
                return None
        else:
            return None

        # check for prometheus process_cpu_usage threshold
        query = """
            SELECT * FROM prometheus_metrics
            WHERE metric_name = ?
            AND timestamp >= ?
            AND timestamp < ?
            ORDER BY timestamp DESC
            LIMIT 1
        """

        params = (
            "process_cpu_usage",
            self.time_window_back(180),
            self.time_window_forward(180)
        )

        row = self.database.execute(query, params)

        if row:
            if row[0]["metric_value"] < PROMETHEUS_CPU_USAGE_THRESHOLD:
                return None
        else:
            return None

        return True

    def anomaly_4(self):
        # if the terminal handler log isn't a network timeout then this isn't an A4
        if self.row["event_type"] != "STARTUP" and self.row["exception_class"] != "java.lang.OutOfMemoryError":
            return None
        
        # count how many recent STARTUPs there've been, check if it's under 3
        query = """
            SELECT COUNT(DISTINCT container_id) AS count FROM terminal_handler_log
            WHERE event_type = ?
            AND timestamp >= ?
            AND timestamp < ?
        """
        params = (
            "STARTUP",
            self.time_window_back(600),
            self.time_window_forward(600)
        )
        
        row = self.database.execute(query, params)

        if row:
            if row[0]["count"] < 2:
                return None
            
        # count recent FATAL OutOfMemoryErrors
        query = """
            SELECT COUNT(*) AS count FROM terminal_handler_log
            WHERE exception_class = ?
            AND log_level = ?
            AND timestamp >= ?
            AND timestamp < ?
        """
        params = (
            "java.lang.OutOfMemoryError",
            "FATAL",
            self.time_window_back(600),
            self.time_window_forward(600)
        )
        
        row = self.database.execute(query, params)

        if row:
            if row[0]["count"] < 2:
                return None
            
        # check if the gcp container/restart_count is going up recently
        query = """
            SELECT MAX(metric_value) - MIN(metric_value) AS delta
            FROM gcp_cloud_metrics
            WHERE metric_name = ?
            AND timestamp >= ?
            AND timestamp < ?
        """
        params = (
            "container/restart_count",
            self.time_window_back(600),
            self.time_window_forward(600)
        )
            
        row = self.database.execute(query, params)

        if row:
            if row[0]["delta"] == 0:
                return None
            
        return True