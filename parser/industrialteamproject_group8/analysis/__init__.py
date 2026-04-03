from pathlib import Path
import importlib

for file in Path(__file__).parent.glob("*.py"):
    if file.name != "__init__.py":
        importlib.import_module(f"{__package__}.{file.stem}")