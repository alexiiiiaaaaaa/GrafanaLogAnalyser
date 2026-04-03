use crate::db::schema::{INSERT_ANOMALY, INSERT_GCP};
use crate::models::GcpCloudMetric;
use rusqlite::{params, Connection};

pub fn parse_and_insert(
    conn: &Connection,
    file_path: &str,
) -> Result<usize, Box<dyn std::error::Error>> {
    println!("Parsing: {}", file_path);

    let mut reader = csv::Reader::from_path(file_path)?;
    let tx = conn.unchecked_transaction()?;

    let mut inserted = 0;
    let mut anomaly_count = 0;

    for result in reader.deserialize::<GcpCloudMetric>() {
        let record = match result {
            Ok(r) => r,
            Err(e) => {
                eprintln!("   Skipping malformed row: {}", e);
                continue;
            }
        };

        tx.execute(
            INSERT_GCP,
            params![
                record.timestamp,
                record.resource_type,
                record.resource_id,
                record.metric_name,
                record.metric_value,
                record.cpu_usage_percent,
                record.memory_usage_bytes,
                record.anomaly,
            ],
        )?;
        inserted += 1;

        if let Some(anomaly_code) = &record.anomaly {
            tx.execute(
                INSERT_ANOMALY,
                params![
                    record.timestamp,
                    Option::<String>::None, // GCP не має atm_id
                    anomaly_code,
                    "gcp_cloud_metrics",
                    0,
                    classify_severity(anomaly_code),
                    format!(
                        "Resource: {:?}, metric: {:?}={:?}",
                        record.resource_id, record.metric_name, record.metric_value
                    ),
                ],
            )?;
            anomaly_count += 1;
        }
    }

    tx.commit()?;
    println!("   Inserted: {} records", inserted);
    println!("   Anomalies: {}", anomaly_count);
    Ok(inserted)
}

fn classify_severity(code: &str) -> &'static str {
    match code {
        "A3" => "WARNING",
        "A4" => "CRITICAL",
        _ => "INFO",
    }
}
