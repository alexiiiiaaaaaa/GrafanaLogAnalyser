use serde::{Deserialize, Serialize};

// ── Kafka ATM Metrics ──────────────────────────────────────────
#[derive(Debug, Deserialize, Serialize)]
pub struct KafkaAtmMetric {
    pub timestamp: String,
    pub event_id: String,
    pub correlation_id: Option<String>,
    pub atm_id: String,
    pub atm_status: Option<String>,
    pub transaction_rate_tps: Option<f64>,
    pub response_time_ms: Option<i64>,
    pub transaction_volume: Option<i64>,
    pub transaction_success_rate: Option<f64>,
    pub transaction_failure_reason: Option<String>,
    pub failure_count: Option<i64>,
    pub window_duration_seconds: Option<i64>,
    pub kafka_partition: Option<i64>,
    pub kafka_offset: Option<i64>,
    #[serde(rename = "_anomaly")]
    pub anomaly: Option<String>,
}

// ── Windows OS Metrics ─────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct WindowsOsMetric {
    pub timestamp: String,
    pub atm_id: String,
    pub hostname: String,
    pub os_version: Option<String>,
    pub cpu_usage_percent: Option<f64>,
    pub memory_used_mb: Option<f64>,
    pub memory_total_mb: Option<f64>,
    pub memory_usage_percent: Option<f64>,
    pub disk_read_bytes_per_sec: Option<f64>,
    pub disk_write_bytes_per_sec: Option<f64>,
    pub disk_free_gb: Option<f64>,
    pub network_bytes_sent_per_sec: Option<f64>,
    pub network_bytes_recv_per_sec: Option<f64>,
    pub network_errors: Option<i64>,
    pub process_count: Option<i64>,
    pub system_uptime_seconds: Option<i64>,
    pub event_log_errors_last_min: Option<i64>,
    #[serde(rename = "_anomaly")]
    pub anomaly: Option<String>,
}

// ── GCP Cloud Metrics ──────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct GcpCloudMetric {
    pub timestamp: String,
    pub project_id: Option<String>,
    pub resource_type: Option<String>,
    pub resource_id: Option<String>,
    pub zone: Option<String>,
    pub metric_name: Option<String>,
    pub metric_value: Option<f64>,
    pub metric_unit: Option<String>,
    pub cpu_usage_percent: Option<f64>,
    pub memory_usage_bytes: Option<i64>,
    pub memory_limit_bytes: Option<i64>,
    pub network_ingress_bytes: Option<i64>,
    pub network_egress_bytes: Option<i64>,
    pub restart_count: Option<i64>,
    pub label_app: Option<String>,
    pub label_env: Option<String>,
    pub label_version: Option<String>,
    #[serde(rename = "_anomaly")]
    pub anomaly: Option<String>,
}

// ── Prometheus Metrics ─────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct PrometheusMetric {
    pub timestamp: String,
    pub metric_name: String,
    pub metric_type: Option<String>,
    #[serde(deserialize_with = "flexible_float::deserialize")]
    pub metric_value: Option<f64>,
    pub service_name: Option<String>,
    pub pod_name: Option<String>,
    pub container_id: Option<String>,
    pub label_area: Option<String>,
    pub label_env: Option<String>,
    pub help_text: Option<String>,
    #[serde(rename = "_anomaly")]
    pub anomaly: Option<String>,
}

// ── Hardware Sensor Log ────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct HardwareSensorLog {
    pub timestamp: String,
    pub atm_id: String,
    pub correlation_id: Option<String>,
    pub component: Option<String>,
    pub event_type: Option<String>,
    pub severity: Option<String>,
    pub message: Option<String>,
    pub metric_name: Option<String>,
    pub metric_value: Option<f64>,
    pub metric_unit: Option<String>,
    pub threshold_value: Option<f64>,
    pub firmware_version: Option<String>,
    #[serde(rename = "_anomaly")]
    pub anomaly: Option<String>,
}

// ── ATM Application Log ────────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct AtmApplicationLog {
    pub timestamp: String,
    pub log_level: Option<String>,
    pub atm_id: Option<String>,
    pub location_code: Option<String>,
    pub session_id: Option<String>,
    pub correlation_id: Option<String>,
    pub transaction_id: Option<String>,
    pub event_type: Option<String>,
    pub message: Option<String>,
    pub component: Option<String>,
    pub thread_id: Option<i64>,
    pub response_time_ms: Option<i64>,
    pub error_code: Option<String>,
    pub error_detail: Option<String>,
    pub atm_status: Option<String>,
    pub os_version: Option<String>,
    pub app_version: Option<String>,
    #[serde(rename = "_anomaly")]
    pub anomaly: Option<String>,
}

// ── Terminal Handler Log ───────────────────────────────────────
#[derive(Debug, Deserialize)]
pub struct TerminalHandlerLog {
    pub timestamp: String,
    pub log_level: Option<String>,
    pub service_name: Option<String>,
    pub service_version: Option<String>,
    pub container_id: Option<String>,
    pub pod_name: Option<String>,
    pub correlation_id: Option<String>,
    pub transaction_id: Option<String>,
    pub atm_id: Option<String>,
    pub event_type: Option<String>,
    pub message: Option<String>,
    pub logger_name: Option<String>,
    pub thread_name: Option<String>,
    pub response_time_ms: Option<i64>,
    pub http_status_code: Option<i64>,
    pub exception_class: Option<String>,
    #[serde(rename = "_anomaly")]
    pub anomaly: Option<String>,
}

// Власний десеріалізатор для полів де може бути не-числове значення
pub mod flexible_float {
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = Option::<String>::deserialize(deserializer)?;
        match s {
            None => Ok(None),
            Some(ref val) if val.is_empty() || val == "NaN" || val == "null" => Ok(None),
            Some(val) => val.parse::<f64>().map(Some).or(Ok(None)),
        }
    }
}
