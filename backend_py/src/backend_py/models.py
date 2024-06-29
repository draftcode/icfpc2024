import datetime

from pydantic import BaseModel
from sqlmodel import Field, Index, SQLModel


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


class ScoreboardRow(BaseModel):
    isYou: bool
    # [ranking: number, team: str, ... number]
    values: list[int | str | None]


class ScoreboardLog(SQLModel, table=True):
    id: int | None = Field(default=None, primary_key=True)
    updated: datetime.datetime
    total_score_row: str | None = None
    lambdaman_score_row: str | None = None
    spaceship_score_row: str | None = None
    threed_score_row: str | None = None
    efficiency_score_row: str | None = None


class ScoreParseResult(BaseModel):
    """get XXXで得られるスコア"""

    lambdaman_parsed: datetime.datetime
    lambdaman_score: list[tuple[int | None, int | None]]

    spaceship_parsed: datetime.datetime
    spaceship_score: list[tuple[int | None, int | None]]

    threed_parsed: datetime.datetime
    threed_score: list[tuple[int | None, int | None]]

    efficiency_parsed: datetime.datetime
    efficiency_score: list[tuple[int | None, int | None]]


class ScoreParseResultLog(SQLModel, table=True):
    id: int | None = Field(default=None, primary_key=True)
    updated: datetime.datetime
    score_parse_result: str | None = None
