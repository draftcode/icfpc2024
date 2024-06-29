import datetime
from typing import Sequence

import httpx
from backend_rs import decode_message  # type: ignore
from fastapi import Body, FastAPI, HTTPException, Query
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import PlainTextResponse, RedirectResponse
from pydantic import BaseModel
from sqlmodel import select

from .config import settings
from .deps import SessionDep
from .models import (
    CommunicationLog,
    ScoreboardLog,
    ScoreboardRow,
    ScoreParseResult,
    ScoreParseResultLog,
)

app = FastAPI()
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_methods=["*"],
    allow_headers=["*"],
)

http_client = httpx.Client(headers={"Authorization": f"Bearer {settings.API_TOKEN}"})


@app.get("/", include_in_schema=False)
async def root():
    return RedirectResponse("/docs")


@app.post("/communicate", response_class=PlainTextResponse)
async def communicate(
    session: SessionDep, body: str = Body(..., media_type="text/plain")
) -> str:
    resp = http_client.post("https://boundvariable.space/communicate", content=body)
    if not resp.is_success:
        raise HTTPException(status_code=resp.status_code, detail=resp.text)

    resp_str = resp.text
    req_str = decode_message(body)
    log = CommunicationLog(
        created=datetime.datetime.now(),
        request=body,
        response=resp_str,
        decoded_request_prefix=req_str[:100],
        decoded_request=req_str,
        decoded_response=decode_message(resp_str),
    )
    session.add(log)
    session.commit()

    return resp_str


@app.get("/communications")
async def communications(
    session: SessionDep,
    decoded_request: str | None = None,
    decoded_request_prefix: str | None = None,
    offset: int = 0,
    limit: int = Query(default=10),
) -> Sequence[CommunicationLog]:
    q = select(CommunicationLog)
    if decoded_request:
        q = q.where(CommunicationLog.decoded_request_prefix == decoded_request)
    if decoded_request_prefix:
        q = q.where(
            CommunicationLog.decoded_request_prefix.like(decoded_request_prefix + "%")  # type: ignore
        )

    return session.exec(
        q.order_by(CommunicationLog.id.desc())  # type: ignore
        .offset(offset)
        .limit(limit)
    ).all()


class ProblemRank(BaseModel):
    id: int
    rank: int | None
    our_score: int | None
    best_score: int | None


class ProblemSetRank(BaseModel):
    updated: datetime.datetime
    rank: int
    problems: list[ProblemRank]


class TeamRankResponse(BaseModel):
    scoreboard_last_updated: datetime.datetime
    total_rank: int

    lambdaman: ProblemSetRank
    spaceship: ProblemSetRank
    threed: ProblemSetRank
    efficiency: ProblemSetRank


@app.get("/team_rank")
async def team_rank(session: SessionDep) -> TeamRankResponse:
    scoreboard_log = session.scalar(select(ScoreboardLog))
    if not scoreboard_log:
        raise HTTPException(status_code=404, detail="ScoreboardLog not found")
    parsed_result_log = session.scalar(select(ScoreParseResultLog))
    if not parsed_result_log or not parsed_result_log.score_parse_result:
        raise HTTPException(status_code=404, detail="ScoreParseResultLog not found")

    total_score_row = ScoreboardRow.model_validate_json(
        scoreboard_log.total_score_row or ""
    )
    lambdaman_score_row = ScoreboardRow.model_validate_json(
        scoreboard_log.lambdaman_score_row or ""
    )
    spaceship_score_row = ScoreboardRow.model_validate_json(
        scoreboard_log.spaceship_score_row or ""
    )
    threed_score_row = ScoreboardRow.model_validate_json(
        scoreboard_log.threed_score_row or ""
    )
    efficiency_score_row = ScoreboardRow.model_validate_json(
        scoreboard_log.efficiency_score_row or ""
    )
    parsed_result = ScoreParseResult.model_validate_json(
        parsed_result_log.score_parse_result
    )
    return TeamRankResponse(
        scoreboard_last_updated=scoreboard_log.updated,
        total_rank=total_score_row.values[0],  # type: ignore
        lambdaman=_to_problem_set_rank(
            lambdaman_score_row,
            parsed_result.lambdaman_parsed,
            parsed_result.lambdaman_score,
        ),
        spaceship=_to_problem_set_rank(
            spaceship_score_row,
            parsed_result.spaceship_parsed,
            parsed_result.spaceship_score,
        ),
        threed=_to_problem_set_rank(
            threed_score_row, parsed_result.threed_parsed, parsed_result.threed_score
        ),
        efficiency=_to_problem_set_rank(
            efficiency_score_row,
            parsed_result.efficiency_parsed,
            parsed_result.efficiency_score,
        ),
    )


def _to_problem_set_rank(
    scoreboard: ScoreboardRow,
    updated: datetime.datetime,
    scores: list[tuple[int | None, int | None]],
) -> ProblemSetRank:
    problems = []
    for i, (our_score, best_score) in enumerate(scores):
        rank = scoreboard.values[i + 2] if i + 2 < len(scoreboard.values) else None
        problems.append(
            ProblemRank(
                id=i + 1,
                rank=rank,  # type: ignore
                our_score=our_score,
                best_score=best_score,
            )
        )
    return ProblemSetRank(updated=updated, rank=scoreboard.values[0], problems=problems)  # type: ignore
