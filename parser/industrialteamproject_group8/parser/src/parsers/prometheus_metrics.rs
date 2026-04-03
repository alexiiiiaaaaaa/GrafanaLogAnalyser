use crate::db::schema::{INSERT_ANOMALY, INSERT_PROMETHEUS};
use crate::models::PrometheusMetric;
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

    for result in reader.deserialize::<PrometheusMetric>() {
        let record = match result {
            Ok(r) => r,
            Err(e) => {
                eprintln!("   Skipping malformed row: {}", e);
                continue;
            }
        };

        tx.execute(
            INSERT_PROMETHEUS,
            params![
                record.timestamp,
                record.metric_name,
                record.metric_type,
                record.metric_value,
                record.service_name,
                record.pod_name,
                record.anomaly,
            ],
        )?;
        inserted += 1;

        if let Some(anomaly_code) = &record.anomaly {
            tx.execute(
                INSERT_ANOMALY,
                params![
                    record.timestamp,
                    Option::<String>::None, // Prometheus не має atm_id
                    anomaly_code,
                    "prometheus_metrics",
                    0,
                    classify_severity(anomaly_code),
                    format!("Metric: {} = {:?}", record.metric_name, record.metric_value),
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
        "A7_MALFORMED" => "WARNING",
        _ => "INFO",
    }
}
