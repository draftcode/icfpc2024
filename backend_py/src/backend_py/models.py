from sqlmodel import Field, SQLModel, Index
import datetime

class CommunicationLog(SQLModel, table=True):
    id: int | None = Field(default=None, primary_key=True)
    created: datetime.datetime
    request: str
    response: str
    # decoded_requestが大きすぎてindexがはれないため、先頭100文字を切り取ったフィールド
    # をつくってそこにIndexをはります。
    decoded_request_prefix: str | None = Field(default=None, index=True)
    decoded_request: str | None
    decoded_response: str | None = None

Index(
    "ix_communicationlog_decoded_request_prefix_id",
    CommunicationLog.decoded_request_prefix,  # type: ignore
    CommunicationLog.id.desc(),  # type: ignore
)
