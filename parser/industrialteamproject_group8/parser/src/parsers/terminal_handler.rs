use crate::db::schema::{INSERT_ANOMALY, INSERT_FACT_METRIC, INSERT_TERMINAL};
use crate::models::TerminalHandlerLog;
use rusqlite::{params, Connection};
use std::fs;

pub fn parse_and_insert(
    conn: &Connection,
    file_path: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    println!("Parsing: {}", file_path);

    let content = fs::read_to_string(file_path)?;
    let records: Vec<TerminalHandlerLog> = serde_json::from_str(&content)?;
    let total = records.len();
    let tx = conn.unchecked_transaction()?;

    let mut inserted = 0;
    let mut anomaly_count = 0;

    for record in &records {
        tx.execute(
            INSERT_TERMINAL,
            params![
                record.timestamp,
                record.log_level,
                record.event_type,
                record.message,
                record.container_id,
                record.correlation_id,
                record.transaction_id,
                record.atm_id,
                record.response_time_ms,
                record.http_status_code,
                record.exception_class,
                record.anomaly,
            ],
        )?;
        inserted += 1;

        // fact_metrics — response_time і http_status
        if let Some(rt) = record.response_time_ms {
            tx.execute(
                INSERT_FACT_METRIC,
                params![
                    record.timestamp,
                    record.atm_id,
                    "terminal_handler",
                    "response_time_ms",
                    rt as f64,
                    record.anomaly,
                ],
            )?;
        }

        if let Some(status) = record.http_status_code {
            tx.execute(
                INSERT_FACT_METRIC,
                params![
                    record.timestamp,
                    record.atm_id,
                    "terminal_handler",
                    "http_status_code",
                    status as f64,
                    record.anomaly,
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
                    "terminal_handler_log",
                    0,
                    classify_severity(anomaly_code),
                    format!(
                        "Event: {:?}, HTTP: {:?}, exception: {:?}",
                        record.event_type, record.http_status_code, record.exception_class
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
        "A3" => "WARNING",
        "A4" => "CRITICAL",
        _ => "INFO",
    }
}
