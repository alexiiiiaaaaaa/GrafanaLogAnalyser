use crate::parsers::atm_app_log;
use crate::parsers::gcp_metrics;
use crate::parsers::hardware_sensor;
use crate::parsers::kafka_metrics;
use crate::parsers::prometheus_metrics;
use crate::parsers::terminal_handler;
use crate::parsers::windows_metrics;
use rusqlite::Connection;
use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;

pub struct FileWatcher {
    files: HashMap<String, SystemTime>,
    conn: Connection,
}

impl FileWatcher {
    pub fn new(conn: Connection) -> Self {
        Self {
            files: HashMap::new(),
            conn,
        }
    }

    pub fn watch(&mut self, interval_secs: u64) {
        println!(
            "👁 Watching data/ for changes (every {}s)...\n",
            interval_secs
        );

        loop {
            self.check_files();
            std::thread::sleep(std::time::Duration::from_secs(interval_secs));
        }
    }

    fn check_files(&mut self) {
        check_file(
            &mut self.conn,
            &mut self.files,
            "data/kafka_atm_metrics_stream.json",
            "kafka",
            kafka_metrics::parse_and_insert,
        );
        check_file(
            &mut self.conn,
            &mut self.files,
            "data/windows_os_metrics.csv",
            "windows_os",
            windows_metrics::parse_and_insert,
        );
        check_file(
            &mut self.conn,
            &mut self.files,
            "data/gcp_cloud_metrics.csv",
            "gcp",
            gcp_metrics::parse_and_insert,
        );
        check_file(
            &mut self.conn,
            &mut self.files,
            "data/prometheus_metrics.csv",
            "prometheus",
            prometheus_metrics::parse_and_insert,
        );
        check_file(
            &mut self.conn,
            &mut self.files,
            "data/atm_hardware_sensor_log.json",
            "hardware_sensor",
            hardware_sensor::parse_and_insert,
        );
        check_file(
            &mut self.conn,
            &mut self.files,
            "data/atm_application_log.json",
            "atm_application",
            atm_app_log::parse_and_insert,
        );
        check_file(
            &mut self.conn,
            &mut self.files,
            "data/terminal_handler_app_log.json",
            "terminal_handler",
            terminal_handler::parse_and_insert,
        );
    }
}

fn check_file<F>(
    conn: &mut Connection,
    files: &mut HashMap<String, SystemTime>,
    file_path: &str,
    source: &str,
    parser: F,
) where
    F: Fn(&Connection, &str) -> Result<usize, Box<dyn std::error::Error>>,
{
    if let Ok(metadata) = fs::metadata(file_path) {
        if let Ok(modified) = metadata.modified() {
            let last_modified = files.get(file_path).copied();

            if last_modified.is_none() || last_modified.unwrap() != modified {
                files.insert(file_path.to_string(), modified);

                println!("📄 File changed: {}", file_path);

                clear_source_data(conn, source);

                match parser(conn, file_path) {
                    Ok(count) => println!("   ✓ Updated: {} rows\n", count),
                    Err(e) => println!("   ✗ Error: {}\n", e),
                }
            }
        }
    }
}

fn clear_source_data(conn: &mut Connection, source: &str) {
    let tables = match source {
        "kafka" => vec!["kafka_atm_metrics"],
        "windows_os" => vec!["windows_os_metrics"],
        "gcp" => vec!["gcp_cloud_metrics"],
        "prometheus" => vec!["prometheus_metrics"],
        "hardware_sensor" => vec!["hardware_sensor_log"],
        "atm_application" => vec!["atm_application_log"],
        "terminal_handler" => vec!["terminal_handler_log"],
        _ => vec![],
    };

    for table in tables {
        let _ = conn.execute(&format!("DELETE FROM {}", table), []);
    }

    let _ = conn.execute(
        "DELETE FROM anomalies WHERE source_table LIKE ?",
        [format!("%{}%", source)],
    );
    let _ = conn.execute("DELETE FROM fact_metrics WHERE source = ?", [source]);
}
