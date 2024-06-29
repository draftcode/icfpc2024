import datetime

import httpx
from celery import Celery
from pydantic import BaseModel
from sqlmodel import Session, select

import re
from .config import settings
from .db import engine
from .models import (
    ScoreboardLog,
    ScoreboardRow,
    ScoreParseResult,
    ScoreParseResultLog,
    CommunicationLog,
)

app = Celery("icfpc2024", broker="redis://localhost")
app.conf.update(task_ignore_result=True)
app.conf.beat_schedule = {
    "update_parsed_result": {
        # Parse get XXX responses. This is a local op, so run every 60 seconds.
        "task": "backend_py.tasks.update_parsed_result",
        "schedule": 60.0,
    },
    "update_scoreboard": {
        # Fetch from the official scoreboard. This is a remote op, so run every 5 minutes.
        "task": "backend_py.tasks.update_scoreboard",
        "schedule": 300.0,
    },
}

http_client = httpx.Client(headers={"Authorization": f"Bearer {settings.API_TOKEN}"})


class ScoreboardResponse(BaseModel):
    rows: list[ScoreboardRow]


@app.task
def update_scoreboard():
    total_score = _get_our_score("https://boundvariable.space/scoreboard")
    lambdaman_score = _get_our_score("https://boundvariable.space/scoreboard/lambdaman")
    spaceship_score = _get_our_score("https://boundvariable.space/scoreboard/spaceship")
    threed_score = _get_our_score("https://boundvariable.space/scoreboard/3d")
    efficiency_score = _get_our_score(
        "https://boundvariable.space/scoreboard/efficiency"
    )

    with Session(engine) as session:
        log = session.scalar(select(ScoreboardLog))
        if not log:
            log = ScoreboardLog(updated=datetime.datetime.now())
            session.add(log)
        log.updated = datetime.datetime.now()
        log.total_score_row = total_score.model_dump_json()
        log.lambdaman_score_row = lambdaman_score.model_dump_json()
        log.spaceship_score_row = spaceship_score.model_dump_json()
        log.threed_score_row = threed_score.model_dump_json()
        log.efficiency_score_row = efficiency_score.model_dump_json()
        session.commit()


@app.task
def update_parsed_result():
    with Session(engine) as session:
        lambdaman_parsed, lambdaman_score = _parse_lambdaman(session)
        spaceship_parsed, spaceship_score = _parse_spaceship(session)
        threed_parsed, threed_score = _parse_3d(session)
        efficiency_parsed, efficiency_score = _parse_efficiency(session)
        result = ScoreParseResult(
            lambdaman_parsed=lambdaman_parsed,
            lambdaman_score=lambdaman_score,
            spaceship_parsed=spaceship_parsed,
            spaceship_score=spaceship_score,
            threed_parsed=threed_parsed,
            threed_score=threed_score,
            efficiency_parsed=efficiency_parsed,
            efficiency_score=efficiency_score,
        )
        log = session.scalar(select(ScoreParseResultLog))
        if not log:
            log = ScoreParseResultLog(updated=datetime.datetime.now())
            session.add(log)
        log.updated = datetime.datetime.now()
        log.score_parse_result = result.model_dump_json()
        session.commit()


def _get_our_score(url: str) -> ScoreboardRow:
    raw_resp = http_client.get(url)
    raw_resp.raise_for_status()
    resp = ScoreboardResponse.model_validate(raw_resp.json())
    for row in resp.rows:
        if not row.isYou:
            continue
        return row
    raise Exception("not found")


def _parse_lambdaman(
    session: Session,
) -> tuple[datetime.datetime, list[tuple[int | None, int | None]]]:
    log = session.scalar(
        select(CommunicationLog)
        .where(CommunicationLog.decoded_request_prefix == "get lambdaman")
        .order_by(CommunicationLog.id.desc())  # type: ignore
    )
    if not log or not log.decoded_response:
        raise Exception("not found")
    return log.created, _extract_scores("lambdaman", log.decoded_response)


def _parse_spaceship(
    session: Session,
) -> tuple[datetime.datetime, list[tuple[int | None, int | None]]]:
    log = session.scalar(
        select(CommunicationLog)
        .where(CommunicationLog.decoded_request_prefix == "get spaceship")
        .order_by(CommunicationLog.id.desc())  # type: ignore
    )
    if not log or not log.decoded_response:
        raise Exception("not found")
    return log.created, _extract_scores("spaceship", log.decoded_response)


def _parse_3d(
    session: Session,
) -> tuple[datetime.datetime, list[tuple[int | None, int | None]]]:
    log = session.scalar(
        select(CommunicationLog)
        .where(CommunicationLog.decoded_request_prefix == "get 3d")
        .order_by(CommunicationLog.id.desc())  # type: ignore
    )
    if not log or not log.decoded_response:
        raise Exception("not found")
    return log.created, _extract_scores("3d", log.decoded_response)


def _parse_efficiency(
    session: Session,
) -> tuple[datetime.datetime, list[tuple[int | None, int | None]]]:
    log = session.scalar(
        select(CommunicationLog)
        .where(CommunicationLog.decoded_request_prefix == "get efficiency")
        .order_by(CommunicationLog.id.desc())  # type: ignore
    )
    if not log or not log.decoded_response:
        raise Exception("not found")

    ret = []
    for line in log.decoded_response.splitlines():
        if not line.startswith("* [efficiency"):
            continue
        if "You solved it" in line:
            ret.append((1, 1))
        elif "At least one other team solved it." in line:
            ret.append((0, 1))
        else:
            ret.append((0, 0))
    return log.created, ret


def _extract_scores(prefix: str, decoded: str) -> list[tuple[int | None, int | None]]:
    ret = []
    for line in decoded.splitlines():
        if not line.startswith("* [" + prefix):
            continue
        m = re.search("Your score: (\\d+)", line)
        our_score = None
        if m:
            our_score = int(m.group(1))

        m = re.search("Best score: (\\d+)", line)
        best_score = None
        if m:
            best_score = int(m.group(1))
        ret.append((our_score, best_score))
    return ret
