use crate::db::schema::{INSERT_ANOMALY, INSERT_FACT_METRIC, INSERT_HARDWARE};
use crate::models::HardwareSensorLog;
use rusqlite::{params, Connection};
use std::fs;

pub fn parse_and_insert(
    conn: &Connection,
    file_path: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    println!("Parsing: {}", file_path);

    let content = fs::read_to_string(file_path)?;
    let records: Vec<HardwareSensorLog> = serde_json::from_str(&content)?;
    let total = records.len();
    let tx = conn.unchecked_transaction()?;

    let mut inserted = 0;
    let mut anomaly_count = 0;

    for record in &records {
        tx.execute(
            INSERT_HARDWARE,
            params![
                record.timestamp,
                record.atm_id,
                record.component,
                record.event_type,
                record.severity,
                record.metric_name,
                record.metric_value,
                record.threshold_value,
                record.anomaly,
            ],
        )?;
        inserted += 1;

        // fact_metrics — числові значення датчиків
        if let (Some(name), Some(value)) = (&record.metric_name, record.metric_value) {
            tx.execute(
                INSERT_FACT_METRIC,
                params![
                    record.timestamp,
                    record.atm_id,
                    "hardware_sensor",
                    name,
                    value,
                    record.anomaly,
                ],
            )?;
        }

        // dim_atm
        tx.execute(
            "INSERT OR IGNORE INTO dim_atm (atm_id, first_seen, last_seen)
             VALUES (?1, ?2, ?2)",
            params![record.atm_id, record.timestamp],
        )?;

        // anomalies
        if let Some(anomaly_code) = &record.anomaly {
            tx.execute(
                INSERT_ANOMALY,
                params![
                    record.timestamp,
                    record.atm_id,
                    anomaly_code,
                    "hardware_sensor_log",
                    0,
                    classify_severity(anomaly_code),
                    format!(
                        "Component: {:?}, value: {:?}, threshold: {:?}",
                        record.component, record.metric_value, record.threshold_value
                    ),
                ],
            )?;
            anomaly_count += 1;
        }
    }

    tx.commit()?;
    println!("   Inserted: {}/{} records", inserted, total);
    println!("   Anomalies: {}", anomaly_count);
    Ok(inserted)
}

fn classify_severity(code: &str) -> &'static str {
    match code {
        "A2" => "WARNING",
        _ => "INFO",
    }
}
