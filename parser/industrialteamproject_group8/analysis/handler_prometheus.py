# subclass responsible for checking for anomalies which are relevant for prometheus logs: A7
from .handler_generic import GenericHandler
import re

class PrometheusHandler(GenericHandler):
    table = "prometheus_metrics"

    def identify(self):
        alerts = []

        if self.anomaly_7():
            self.row.update({"detected_anomaly": "A7: Malformed Prometheus record"})
            alerts.append(self.row)
          
        return alerts
    
    def anomaly_7(self):
        if isinstance(self.row["metric_value"], float):
            return None
        
        return True