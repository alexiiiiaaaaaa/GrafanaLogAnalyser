import subprocess
import time
from analysis.sqlite3_service import SQLite3Service
import logging
import sys

import pydevd_pycharm

# Connect to PyCharm debugger (run this before your main code)
# pydevd_pycharm.settrace(
#     'host.docker.internal',  # On Mac/Windows
#     # '172.17.0.1',          # On Linux, use this instead
#     port=7777,
#     stdoutToServer=True,
#     stderrToServer=True,
#     suspend=False  # Set to True to wait for debugger before continuing
# )

# Configure logging to be more verbose
logging.basicConfig(
    level=logging.DEBUG,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    handlers=[
        logging.StreamHandler(sys.stdout)
    ]
)


SQLITE_FP = "/app/sqlite/atm_data.db"
CONFIG_FP = "config.txt"
database = SQLite3Service(SQLITE_FP, CONFIG_FP)

database.start_listener()

# Keep the main thread alive
try:
    # Wait forever (or until Ctrl+C)
    while True:
        time.sleep(1)
except KeyboardInterrupt:
    print("Shutting down...")
    database.stop_listener()  # If you have this method