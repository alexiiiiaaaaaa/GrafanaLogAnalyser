# ATM Parser

SynthBank ATM log parser. Reads 7 data sources and loads them into an SQLite database (`atm_data.db`).

## Overview

This application monitors the `data/` directory and automatically re-parses files when they are updated, keeping the database synchronized with the latest data.

### Architecture

```
┌─────────────────────────────────────────────────────────┐
│                      main.rs                            │
│   Initializes DB → starts FileWatcher (infinite loop)   │
└─────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────┐
│                    watcher.rs                           │
│   Polls data/ every 5s → checks file modification time  │
│   On change: DELETE old data → INSERT new data          │
└─────────────────────────────────────────────────────────┘
                            │
         ┌──────────────────┼──────────────────┐
         ▼                  ▼                  ▼
   ┌──────────┐      ┌──────────┐       ┌──────────┐
   │  parsers │      │   db/    │       │  models  │
   │ *.rs     │      │ schema   │       │  structs │
   └──────────┘      └──────────┘       └──────────┘
```

### Why this design?

1. **Polling instead of inotify**: Simpler implementation, works cross-platform, no kernel dependencies
2. **5-second interval**: Fast enough for most use cases, not too CPU-intensive
3. **DELETE + INSERT**: When a file changes, old data is removed and new data inserted (not INSERT OR IGNORE)
4. **WAL mode enabled**: `PRAGMA journal_mode=WAL` provides better write performance and non-blocking reads

## Run

```bash
cargo run
```

The application will:
1. Initialize the SQLite database
2. Seed dimension tables (`dim_source`)
3. Parse all files in `data/` initially
4. Start watching for file changes (every 5 seconds)

Press `Ctrl+C` to stop.

## Data Files

The `data/` folder must contain:
- `kafka_atm_metrics_stream.json` — ATM transaction stream
- `windows_os_metrics.csv` — OS performance metrics per ATM
- `gcp_cloud_metrics.csv` — GCP container metrics
- `prometheus_metrics.csv` — JVM/service metrics
- `atm_hardware_sensor_log.json` — Hardware sensor logs
- `atm_application_log.json` — ATM client application logs
- `terminal_handler_app_log.json` — Backend service logs

## Database

### Staging tables (raw data)
| Table | Entries | Description |
|--------|---------|------|
| kafka_atm_metrics | 8,642 | ATM transactions |
| windows_os_metrics | 8,640 | CPU/RAM/Disk metrics |
| gcp_cloud_metrics | 2,878 | GCP Cloud Metrics |
| prometheus_metrics | 12,952 | Service JVM metrics |
| hardware_sensor_log | 596 | ATM sensors |
| atm_application_log | 3,954 | ATM sensors |
| terminal_handler_log | 8,594 | Backend service logs |

### Analytics tables
| Table | Description |
|--------|------|
| fact_metrics | All numerical metrics from all sources |
| anomalies | All anomalies from all sources |
| dim_atm | List of 6 ATMs |
| dim_source | List of data sources |

## How It Works

When the watcher detects a file change:
1. The file's modification timestamp is compared to the last known timestamp
2. If different → clears the corresponding staging table in the database
3. Clears related entries in `anomalies` and `fact_metrics` tables
4. Re-parses the file and inserts new data

This ensures the database always reflects the current state of the data files.

## Useful queries
```sql
-- All anomalies sorted by time
SELECT timestamp, atm_id, anomaly_code, severity, description
FROM anomalies
ORDER BY timestamp;

-- Average CPU of each ATM
SELECT atm_id, ROUND(AVG(metric_value), 2) as avg_cpu
FROM fact_metrics
WHERE metric_name = 'cpu_usage_percent'
GROUP BY atm_id
ORDER BY avg_cpu DESC;

-- Critical anomalies
SELECT timestamp, atm_id, anomaly_code, source_table
FROM anomalies
WHERE severity = 'CRITICAL'
ORDER BY timestamp;

-- All metrics for a specific ATM
SELECT timestamp, source, metric_name, metric_value
FROM fact_metrics
WHERE atm_id = 'ATM-GB-0001'
ORDER BY timestamp;
```
