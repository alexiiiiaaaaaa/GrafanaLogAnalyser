use crate::db::schema::{INSERT_ANOMALY, INSERT_FACT_METRIC, INSERT_WINDOWS};
use crate::models::WindowsOsMetric;
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

    for result in reader.deserialize::<WindowsOsMetric>() {
        let record = match result {
            Ok(r) => r,
            Err(e) => {
                eprintln!("   Skipping malformed row: {}", e);
                continue;
            }
        };

        // ── Основний INSERT ──────────────────────────────────────
        tx.execute(
            INSERT_WINDOWS,
            params![
                record.timestamp,
                record.atm_id,
                record.hostname,
                record.cpu_usage_percent,
                record.memory_used_mb,
                record.memory_total_mb,
                record.memory_usage_percent,
                record.disk_free_gb,
                record.network_errors,
                record.anomaly,
            ],
        )?;
        inserted += 1;

        // ── fact_metrics — ключові числові метрики ───────────────
        for (name, value) in [
            ("cpu_usage_percent", record.cpu_usage_percent),
            ("memory_used_mb", record.memory_used_mb),
            ("disk_free_gb", record.disk_free_gb),
        ] {
            if let Some(v) = value {
                tx.execute(
                    INSERT_FACT_METRIC,
                    params![
                        record.timestamp,
                        record.atm_id,
                        "windows_os",
                        name,
                        v,
                        record.anomaly,
                    ],
                )?;
            }
        }

        // ── dim_atm — реєструємо банкомат ────────────────────────
        tx.execute(
            "INSERT OR IGNORE INTO dim_atm
             (atm_id, hostname, os_version, first_seen, last_seen)
             VALUES (?1, ?2, ?3, ?4, ?4)",
            params![
                record.atm_id,
                record.hostname,
                record.os_version,
                record.timestamp,
            ],
        )?;

        // ── Аномалії ─────────────────────────────────────────────
        if let Some(anomaly_code) = &record.anomaly {
            tx.execute(
                INSERT_ANOMALY,
                params![
                    record.timestamp,
                    record.atm_id,
                    anomaly_code,
                    "windows_os_metrics",
                    0,
                    classify_severity(anomaly_code),
                    format!(
                        "CPU: {:?}%, Memory: {:?}MB",
                        record.cpu_usage_percent, record.memory_used_mb
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
        "A6" => "WARNING",
        _ => "INFO",
    }
}
