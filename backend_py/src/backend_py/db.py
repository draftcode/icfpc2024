from sqlmodel import create_engine

from . import models  # noqa
from . import config

engine = create_engine(str(config.settings.SQLALCHEMY_DATABASE_URI))
