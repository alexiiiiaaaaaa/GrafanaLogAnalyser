use crate::db::schema::CREATE_TABLES;
use rusqlite::{Connection, Result};

pub mod schema;

pub fn init_db(path: &str) -> Result<Connection> {
    let conn = Connection::open(path)?;

    // Увімкнути WAL для кращої продуктивності
    conn.execute_batch("PRAGMA journal_mode=WAL;")?;

    // Створити всі таблиці
    conn.execute_batch(CREATE_TABLES)?;

    println!("Database initialized: {}", path);
    Ok(conn)
}

pub fn seed_dim_source(conn: &Connection) -> rusqlite::Result<()> {
    let sources = [
        ("kafka", "metrics", "Kafka ATM transaction metrics stream"),
        (
            "windows_os",
            "metrics",
            "Windows OS performance metrics per ATM",
        ),
        ("gcp", "metrics", "Google Cloud Platform container metrics"),
        (
            "prometheus",
            "metrics",
            "Prometheus JVM and service metrics",
        ),
        (
            "hardware_sensor",
            "logs",
            "ATM hardware component sensor logs",
        ),
        ("atm_application", "logs", "ATM client application logs"),
        (
            "terminal_handler",
            "logs",
            "Terminal Handler backend service logs",
        ),
    ];

    for (name, stype, desc) in &sources {
        conn.execute(
            "INSERT OR IGNORE INTO dim_source (source_name, source_type, description)
             VALUES (?1, ?2, ?3)",
            rusqlite::params![name, stype, desc],
        )?;
    }
    Ok(())
}
