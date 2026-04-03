use crate::db::schema::{INSERT_ANOMALY, INSERT_ATM_APP, INSERT_FACT_METRIC};
use crate::models::AtmApplicationLog;
use rusqlite::{params, Connection};
use std::fs;

pub fn parse_and_insert(
    conn: &Connection,
    file_path: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    println!("Parsing: {}", file_path);

    let content = fs::read_to_string(file_path)?;
    let records: Vec<AtmApplicationLog> = serde_json::from_str(&content)?;
    let total = records.len();
    let tx = conn.unchecked_transaction()?;

    let mut inserted = 0;
    let mut anomaly_count = 0;

    for record in &records {
        tx.execute(
            INSERT_ATM_APP,
            params![
                record.timestamp,
                record.atm_id,
                record.log_level,
                record.event_type,
                record.component,
                record.message,
                record.error_code,
                record.error_detail,
                record.response_time_ms,
                record.atm_status,
                record.anomaly,
            ],
        )?;
        inserted += 1;

        // fact_metrics — response_time якщо є
        if let Some(rt) = record.response_time_ms {
            tx.execute(
                INSERT_FACT_METRIC,
                params![
                    record.timestamp,
                    record.atm_id,
                    "atm_application",
                    "response_time_ms",
                    rt as f64,
                    record.anomaly,
                ],
            )?;
        }

        // dim_atm
        if let Some(atm_id) = &record.atm_id {
            tx.execute(
                "INSERT OR IGNORE INTO dim_atm
                 (atm_id, location_code, os_version, first_seen, last_seen)
                 VALUES (?1, ?2, ?3, ?4, ?4)",
                params![
                    atm_id,
                    record.location_code,
                    record.os_version,
                    record.timestamp,
                ],
            )?;
        }

        // anomalies
        if let Some(anomaly_code) = &record.anomaly {
            tx.execute(
                INSERT_ANOMALY,
                params![
                    record.timestamp,
                    record.atm_id,
                    anomaly_code,
                    "atm_application_log",
                    0,
                    classify_severity(anomaly_code),
                    format!(
                        "Event: {:?}, error: {:?}, status: {:?}",
                        record.event_type, record.error_code, record.atm_status
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
        "A1" => "WARNING",
        "A5" => "CRITICAL",
        "A6" => "WARNING",
        _ => "INFO",
    }
}
