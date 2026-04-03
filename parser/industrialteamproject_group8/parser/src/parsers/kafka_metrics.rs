use crate::db::schema::{INSERT_ANOMALY, INSERT_KAFKA};
use crate::models::KafkaAtmMetric;
use rusqlite::{params, Connection};
use std::fs;

pub fn parse_and_insert(
    conn: &Connection,
    file_path: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    println!("Parsing: {}", file_path);

    let content = fs::read_to_string(file_path)?;
    let records: Vec<KafkaAtmMetric> = serde_json::from_str(&content)?;
    let total = records.len();

    // Використовуємо транзакцію для швидкості — замість 8642 окремих INSERT
    let tx = conn.unchecked_transaction()?;

    let mut inserted = 0;
    let mut anomaly_count = 0;

    for record in &records {
        tx.execute(
            INSERT_KAFKA,
            params![
                record.timestamp,
                record.event_id,
                record.atm_id,
                record.atm_status,
                record.transaction_rate_tps,
                record.response_time_ms,
                record.transaction_volume,
                record.transaction_success_rate,
                record.transaction_failure_reason,
                record.failure_count,
                record.anomaly,
            ],
        )?;
        inserted += 1;

        // Якщо є аномалія — записуємо в зведену таблицю
        if let Some(anomaly_code) = &record.anomaly {
            let severity = classify_severity(anomaly_code);
            tx.execute(
                INSERT_ANOMALY,
                params![
                    record.timestamp,
                    record.atm_id,
                    anomaly_code,
                    "kafka_atm_metrics",
                    0, // source_id буде заповнено пізніше
                    severity,
                    format!(
                        "ATM status: {:?}, failure: {:?}",
                        record.atm_status, record.transaction_failure_reason
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

// Класифікуємо серйозність аномалії
fn classify_severity(code: &str) -> &'static str {
    match code {
        "A1" => "WARNING",
        "A2" => "WARNING",
        "A3" => "WARNING",
        "A4" => "CRITICAL",
        "A5" => "CRITICAL",
        "A6" => "WARNING",
        "A7_OUT_OF_ORDER" => "CRITICAL",
        "A7_MALFORMED" => "WARNING",
        _ => "INFO",
    }
}
