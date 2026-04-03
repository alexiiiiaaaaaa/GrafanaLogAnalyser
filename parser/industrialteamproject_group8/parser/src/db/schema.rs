pub const CREATE_TABLES: &str = "
CREATE TABLE IF NOT EXISTS kafka_atm_metrics (
    id INTEGER PRIMARY KEY,
    timestamp TEXT NOT NULL,
    event_id TEXT UNIQUE,
    atm_id TEXT NOT NULL,
    atm_status TEXT,
    transaction_rate_tps REAL,
    response_time_ms INTEGER,
    transaction_volume INTEGER,
    transaction_success_rate REAL,
    transaction_failure_reason TEXT,
    failure_count INTEGER,
    anomaly TEXT,
    created_at TEXT DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS windows_os_metrics (
    id INTEGER PRIMARY KEY,
    timestamp TEXT NOT NULL,
    atm_id TEXT NOT NULL,
    hostname TEXT,
    cpu_usage_percent REAL,
    memory_used_mb REAL,
    memory_total_mb REAL,
    memory_usage_percent REAL,
    disk_free_gb REAL,
    network_errors INTEGER,
    anomaly TEXT
);

CREATE TABLE IF NOT EXISTS gcp_cloud_metrics (
    id INTEGER PRIMARY KEY,
    timestamp TEXT NOT NULL,
    resource_type TEXT,
    resource_id TEXT,
    metric_name TEXT,
    metric_value REAL,
    cpu_usage_percent REAL,
    memory_usage_bytes INTEGER,
    anomaly TEXT
);

CREATE TABLE IF NOT EXISTS prometheus_metrics (
    id INTEGER PRIMARY KEY,
    timestamp TEXT NOT NULL,
    metric_name TEXT NOT NULL,
    metric_type TEXT,
    metric_value REAL,
    service_name TEXT,
    pod_name TEXT,
    anomaly TEXT
);

CREATE TABLE IF NOT EXISTS hardware_sensor_log (
    id INTEGER PRIMARY KEY,
    timestamp TEXT NOT NULL,
    atm_id TEXT NOT NULL,
    component TEXT,
    event_type TEXT,
    severity TEXT,
    metric_name TEXT,
    metric_value REAL,
    threshold_value REAL,
    anomaly TEXT
);

CREATE TABLE IF NOT EXISTS atm_application_log (
    id INTEGER PRIMARY KEY,
    timestamp TEXT NOT NULL,
    atm_id TEXT NOT NULL,
    log_level TEXT,
    event_type TEXT,
    component TEXT,
    message TEXT,
    error_code TEXT,
    error_detail TEXT,
    response_time_ms INTEGER,
    atm_status TEXT,
    anomaly TEXT
);

CREATE TABLE IF NOT EXISTS terminal_handler_log (
    id INTEGER PRIMARY KEY,
    timestamp TEXT NOT NULL,
    log_level TEXT,
    event_type TEXT,
    message TEXT,
    container_id TEXT,
    correlation_id TEXT,
    transaction_id TEXT,
    atm_id TEXT,
    response_time_ms INTEGER,
    http_status_code INTEGER,
    exception_class TEXT,
    anomaly TEXT
);

CREATE TABLE IF NOT EXISTS anomalies (
    id INTEGER PRIMARY KEY,
    timestamp TEXT NOT NULL,
    atm_id TEXT,
    anomaly_code TEXT NOT NULL,
    source_table TEXT NOT NULL,
    source_id INTEGER,
    severity TEXT,
    description TEXT
);

CREATE INDEX IF NOT EXISTS idx_kafka_atm_time ON kafka_atm_metrics(atm_id, timestamp);
CREATE INDEX IF NOT EXISTS idx_windows_atm_time ON windows_os_metrics(atm_id, timestamp);
CREATE INDEX IF NOT EXISTS idx_anomalies_atm ON anomalies(atm_id, timestamp);
CREATE INDEX IF NOT EXISTS idx_anomalies_code ON anomalies(anomaly_code);

CREATE TABLE IF NOT EXISTS dim_atm (
    atm_id TEXT PRIMARY KEY,
    hostname TEXT,
    location_code TEXT,
    os_version TEXT,
    first_seen TEXT,
    last_seen TEXT
);

CREATE TABLE IF NOT EXISTS dim_source (
    source_name TEXT PRIMARY KEY,
    source_type TEXT,
    description TEXT
);

CREATE TABLE IF NOT EXISTS fact_metrics (
    id INTEGER PRIMARY KEY,
    timestamp TEXT NOT NULL,
    atm_id TEXT,
    source TEXT NOT NULL,
    metric_name TEXT NOT NULL,
    metric_value REAL,
    anomaly TEXT
);

CREATE INDEX IF NOT EXISTS idx_fact_metrics_atm
    ON fact_metrics(atm_id, timestamp);
CREATE INDEX IF NOT EXISTS idx_fact_metrics_name
    ON fact_metrics(metric_name);
";

pub const INSERT_KAFKA: &str = "
INSERT OR IGNORE INTO kafka_atm_metrics
(timestamp, event_id, atm_id, atm_status, transaction_rate_tps,
 response_time_ms, transaction_volume, transaction_success_rate,
 transaction_failure_reason, failure_count, anomaly)
VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)
";

pub const INSERT_WINDOWS: &str = "
INSERT INTO windows_os_metrics
(timestamp, atm_id, hostname, cpu_usage_percent, memory_used_mb,
 memory_total_mb, memory_usage_percent, disk_free_gb, network_errors, anomaly)
VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10)
";

pub const INSERT_GCP: &str = "
INSERT INTO gcp_cloud_metrics
(timestamp, resource_type, resource_id, metric_name, metric_value,
 cpu_usage_percent, memory_usage_bytes, anomaly)
VALUES (?1,?2,?3,?4,?5,?6,?7,?8)
";

pub const INSERT_PROMETHEUS: &str = "
INSERT INTO prometheus_metrics
(timestamp, metric_name, metric_type, metric_value,
 service_name, pod_name, anomaly)
VALUES (?1,?2,?3,?4,?5,?6,?7)
";

pub const INSERT_HARDWARE: &str = "
INSERT INTO hardware_sensor_log
(timestamp, atm_id, component, event_type, severity,
 metric_name, metric_value, threshold_value, anomaly)
VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)
";

pub const INSERT_ATM_APP: &str = "
INSERT INTO atm_application_log
(timestamp, atm_id, log_level, event_type, component,
 message, error_code, error_detail, response_time_ms, atm_status, anomaly)
VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)
";

pub const INSERT_TERMINAL: &str = "
INSERT INTO terminal_handler_log
(timestamp, log_level, event_type, message, container_id, correlation_id,
 transaction_id, atm_id, response_time_ms, http_status_code,
 exception_class, anomaly)
VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)
";

pub const INSERT_ANOMALY: &str = "
INSERT INTO anomalies
(timestamp, atm_id, anomaly_code, source_table, source_id, severity, description)
VALUES (?1,?2,?3,?4,?5,?6,?7)
";

pub const INSERT_FACT_METRIC: &str = "
INSERT INTO fact_metrics (timestamp, atm_id, source, metric_name, metric_value, anomaly)
VALUES (?1, ?2, ?3, ?4, ?5, ?6)
";
