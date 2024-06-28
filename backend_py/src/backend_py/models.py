from sqlmodel import Field, SQLModel
import datetime

class CommunicationLog(SQLModel, table=True):
    id: int | None = Field(default=None, primary_key=True)
    created: datetime.datetime
    request: str
    response: str
    decoded_request: str | None = None
    decoded_response: str | None = None
