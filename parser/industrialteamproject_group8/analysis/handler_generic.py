# parent class to all of the different log handlers
from datetime import timedelta, datetime
from . import *

class GenericHandler:
    registry = {}
    
    def __init__(self, database, row):
        self.database = database
        self.row = row

    def __init_subclass__(cls):
        super().__init_subclass__()
        if hasattr(cls, "table"):
            GenericHandler.registry[cls.table] = cls

    @classmethod
    def create(cls, database, row):
        return cls.registry.get(row["table"], DefaultHandler)(database, row)

    def identify(self):
        raise NotImplementedError
    
    def time_window_back(self, window_seconds):
        return (self.row["timestamp"] - timedelta(seconds = window_seconds)).isoformat()
    
    def time_window_forward(self, window_seconds):
        return (self.row["timestamp"] + timedelta(seconds = window_seconds)).isoformat()

class DefaultHandler(GenericHandler):
    def identify(self):
        return []
