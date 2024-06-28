from sqlmodel import Field, SQLModel, Index
import datetime

class CommunicationLog(SQLModel, table=True):
    id: int | None = Field(default=None, primary_key=True)
    created: datetime.datetime
    request: str
    response: str
    decoded_request: str | None = Field(default=None, index=True)
    decoded_response: str | None = None

Index(
    "ix_communicationlog_decoded_request_id",
    CommunicationLog.decoded_request,  # type: ignore
    CommunicationLog.id.desc(),  # type: ignore
)
