from sqlmodel import Field, SQLModel
import datetime

class CommunicationLog(SQLModel, table=True):
    id: int = Field(primary_key=True)
    created: datetime.datetime
    request: str
    response: str
