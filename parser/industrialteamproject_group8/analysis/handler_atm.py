# subclass responsible for checking for anomalies which are relevant for atm application logs, 
from .handler_generic import GenericHandler

class AtmHandler(GenericHandler):
    table = "atm_application_log"

    def identify(self):
        alerts = []

        if self.anomaly_5():
            self.row.update({"detected_anomaly": "A5: High Response Time Spike + Success Rate Drop"})
            alerts.append(self.row)
        
        if self.anomaly_6():
            self.row.update({"detected_anomaly": "A6: OS Memory Pressure, Application Timeout"})
            alerts.append(self.row)

        return alerts
    
    def anomaly_5(self):
        if self.row["event_type"] != "TIMEOUT" or self.row["error_code"] != "ERR-0012":
            return None
        
        # check for a recent failure_count
        query = """
            SELECT * FROM kafka_atm_metrics
            WHERE atm_id = ?
            AND failure_count > ?
            AND timestamp >= ?
            AND timestamp < ?
        """
        params = (
            self.row["atm_id"],
            4,
            self.time_window_back(120),
            self.time_window_forward(120)
        )

        if not self.database.execute(query, params):
            return None
        
        # check for a recent transaction_success_rate
        query = """
            SELECT * FROM kafka_atm_metrics
            WHERE atm_id = ?
            AND transaction_success_rate < ?
            AND timestamp >= ?
            AND timestamp < ?
        """
        params = (
            self.row["atm_id"],
            0.8,
            self.time_window_back(120),
            self.time_window_forward(120)
        )

        if not self.database.execute(query, params):
            return None
        
        # check for a recent response_time_ms
        query = """
            SELECT * FROM kafka_atm_metrics
            WHERE atm_id = ?
            AND response_time_ms > ?
            AND timestamp >= ?
            AND timestamp < ?
        """
        params = (
            self.row["atm_id"],
            1000,
            self.time_window_back(120),
            self.time_window_forward(120)
        )

        if not self.database.execute(query, params):
            return None
        
        return True
    
    def anomaly_6(self):
        if self.row["event_type"] != "TIMEOUT" or "ThreadAbortException" not in self.row["error_detail"]:
            return None
        
        # check for a recent cpu_usage_percent
        query = """
            SELECT * FROM windows_os_metrics
            WHERE atm_id = ?
            AND cpu_usage_percent > ?
            AND timestamp >= ?
            AND timestamp < ?
        """
        params = (
            self.row["atm_id"],
            65,
            self.time_window_back(120),
            self.time_window_forward(120)
        )

        if not self.database.execute(query, params):
            return None
        
         # check for a recent network_errors
        query = """
            SELECT * FROM windows_os_metrics
            WHERE atm_id = ?
            AND network_errors > ?
            AND timestamp >= ?
            AND timestamp < ?
        """
        params = (
            self.row["atm_id"],
            10,
            self.time_window_back(120),
            self.time_window_forward(120)
        )

        # check for large increase in memory_usage_percent
        query = """
            SELECT * FROM windows_os_metrics
            WHERE atm_id = ?
            AND memory_usage_percent > ?
            AND timestamp >= ?
            AND timestamp < ?
        """
        params = (
            self.row["atm_id"],
            80,
            self.time_window_back(120),
            self.time_window_forward(120)
        )

        if self.database.execute(query, params):
            return None
        
        return True