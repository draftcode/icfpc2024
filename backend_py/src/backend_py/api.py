import datetime
import importlib.resources as pkg_resources
from typing import Sequence

from backend_rs import resolve_3d, onestep_3d, encode_message  # type: ignore
from fastapi import Body, FastAPI, HTTPException, Query
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import PlainTextResponse, RedirectResponse
from pydantic import BaseModel
from sqlmodel import select

from .communicate import send_encoded_req
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


@app.get("/", include_in_schema=False)
async def root():
    return RedirectResponse("/docs")


@app.post("/communicate", response_class=PlainTextResponse)
async def communicate(
    session: SessionDep, body: str = Body(..., media_type="text/plain")
) -> str:
    log = send_encoded_req(session, body)
    return log.response


class SubmitRequest(BaseModel):
    plaintext: str | None = None
    icfp: str | None = None


@app.post("/communicate/submit")
async def communicate_submit_plaintext(
    session: SessionDep, body: SubmitRequest
) -> CommunicationLog:
    if body.plaintext is not None:
        return send_encoded_req(session, encode_message(body.plaintext))
    elif body.icfp is not None:
        return send_encoded_req(session, body.icfp)
    raise HTTPException(
        status_code=400, detail="Either plaintext or icfp must be provided"
    )


class ThreedSimulationRequest(BaseModel):
    board: str
    val_a: int
    val_b: int
    turns: int


class ThreedSimulationResult(BaseModel):
    board: str
    output: int | None
    score: int
    error: str | None


@app.post("/simulation/3d")
async def run_3d_simulation(body: ThreedSimulationRequest) -> ThreedSimulationResult:
    try:
        result_board, output, score = onestep_3d(
            body.board, body.val_a, body.val_b, body.turns
        )
        return ThreedSimulationResult(
            board=result_board, output=output, score=score, error=None
        )
    except BaseException as e:
        return ThreedSimulationResult(
            board=body.board, output=None, score=0, error=str(e)
        )


class ThreedResolveRequest(BaseModel):
    board: str


class ThreedResolveResult(BaseModel):
    board: str
    error: str | None


@app.post("/simulation/3d/resolve")
async def resolve_3d_simulation(body: ThreedResolveRequest) -> ThreedResolveResult:
    try:
        result_board = resolve_3d(body.board)
        return ThreedResolveResult(board=result_board, error=None)
    except BaseException as e:
        return ThreedResolveResult(board="", error=str(e))


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


@app.get("/communications/{communication_id}")
async def get_communication(
    session: SessionDep, communication_id: int
) -> CommunicationLog:
    log = session.scalar(
        select(CommunicationLog).where(CommunicationLog.id == communication_id)
    )
    if not log:
        raise HTTPException(status_code=404, detail="CommunicationLog not found")
    return log


@app.get("/solutions/{category}/{problem_id}")
async def solution(
    session: SessionDep,
    category: str,
    problem_id: int,
    offset: int = 0,
    limit: int = Query(default=10),
) -> list[CommunicationLog]:
    if category == "lambdaman":
        prefix = f"solve lambdaman{problem_id}"
    elif category == "spaceship":
        prefix = f"solve spaceship{problem_id}"
    elif category == "3d":
        prefix = f"solve 3d{problem_id}"
    elif category == "efficiency":
        prefix = f"solve efficiency{problem_id}"
    else:
        raise HTTPException(status_code=404, detail="Category not found")
    return session.exec(
        select(CommunicationLog)
        .where(
            CommunicationLog.decoded_request_prefix.op("similar to")(prefix + "( |\n)%")  # type: ignore
        )
        .order_by(CommunicationLog.id.desc())  # type: ignore
        .offset(offset)
        .limit(limit)
    ).all()


class ParsedProblem(BaseModel):
    category: str
    id: int
    content: str


@app.get("/problems/{category}/{problem_id}")
async def problem(category: str, problem_id: int) -> ParsedProblem:
    content = (
        pkg_resources.files("backend_py")
        .joinpath("problems")
        .joinpath(category)
        .joinpath(f"{problem_id}.txt")
        .read_text()
    )
    return ParsedProblem(category=category, id=problem_id, content=content)


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
        if rank == "?":
            rank = 1  # HACK
        problems.append(
            ProblemRank(
                id=i + 1,
                rank=rank,  # type: ignore
                our_score=our_score,
                best_score=best_score,
            )
        )
    rank = scoreboard.values[0]
    if rank == "?":
        rank = 1  # HACK
    return ProblemSetRank(updated=updated, rank=rank, problems=problems)  # type: ignore
