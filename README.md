#### Expected directory structure
```shell
.
├── 2026_NCRAtleos_ProjectBrief.pdf
├── Assets
│   ├── Data-Dictionary
│   │   ├── cross-source-correlation-guide.md
│   │   ├── data-dictionary-atm-application-log.json
│   │   ├── data-dictionary-atm-hardware-sensor-log.json
│   │   ├── data-dictionary-gcp-cloud-metrics.json
│   │   ├── data-dictionary-kafka-atm-metrics-stream.json
│   │   ├── data-dictionary-prometheus-metrics.json
│   │   ├── data-dictionary-terminal-handler-app-log.json
│   │   └── data-dictionary-windows-os-metrics-log.json
│   └── Synthetic-Data
│       ├── 24h-Data-Set
│       │   ├── atm_application_log.json
│       │   ├── atm_hardware_sensor_log.json
│       │   ├── gcp_cloud_metrics.csv
│       │   ├── kafka_atm_metrics_stream.json
│       │   ├── prometheus_metrics.csv
│       │   ├── synthetic_data_generate_large_scale_data.py
│       │   ├── terminal_handler_app_log.json
│       │   └── windows_os_metrics.csv
│       ├── anomaly_detection_guide.md
│       ├── atm_application_log.json
│       ├── atm_hardware_sensor_log.json
│       ├── gcp_cloud_metrics.csv
│       ├── kafka_atm_metrics_stream.json
│       ├── prometheus_metrics.csv
│       ├── terminal_handler_app_log.json
│       └── windows_os_metrics.csv
├── README.md
├── app_configs
│   ├── loki-config.yaml
│   ├── promtail-config.yaml
│   └── telegraf.conf
├── docker-compose.yml
├── grafana
│   └── provisioning
│       ├── dashboards
│       │   ├── anomaly-detection.json
│       │   └── dashboard.yml
│       └── datasources
│           └── datasources.yaml
├── grafana-data
├── log-examples
└── shared-logs
    └── demo.log
13 directories, 34 files
```
#### Enviroment setup:
```shell  
1) Install Docker desktop from:
Windows: https://docs.docker.com/desktop/setup/install/windows-install/ 
Mac: https://docs.docker.com/desktop/setup/install/mac-install/
Linux: https://docs.docker.com/desktop/setup/install/linux/

2) Navigate to project directory 

3) run command:
docker compose up
```

#### Usefull commands:
```shell                                                                                             
docker-compose down

# Remove ALL volumes (including InfluxDB if you want fresh data)
docker volume rm $(docker volume ls -q | grep grafana)

# Restart
docker-compose up -d

# Wait 10 seconds for services to start
sleep 10
```