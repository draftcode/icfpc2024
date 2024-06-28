import httpx
from backend_rs import decode_message  # type: ignore
from fastapi import FastAPI, Body, Query
from fastapi.responses import RedirectResponse, PlainTextResponse
from .config import settings
from .deps import SessionDep
from .models import CommunicationLog
from sqlmodel import select
import datetime
from typing import Sequence

app = FastAPI()
http_client = httpx.Client(headers={"Authorization": f"Bearer {settings.API_TOKEN}"})


@app.get("/", include_in_schema=False)
async def root():
    return RedirectResponse("/docs")


@app.post("/communicate", response_class=PlainTextResponse)
async def communicate(
    session: SessionDep, body: str = Body(..., media_type="text/plain")
) -> str:
    resp = http_client.post("https://boundvariable.space/communicate", content=body)
    resp.raise_for_status()

    resp_str = resp.text

    log = CommunicationLog(
        created=datetime.datetime.now(),
        request=body,
        response=resp_str,
        decoded_request=decode_message(body),
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
        q = q.where(CommunicationLog.decoded_request == decoded_request)
    if decoded_request_prefix:
        q = q.where(CommunicationLog.decoded_request.like(decoded_request_prefix + "%"))  # type: ignore

    return session.exec(
        q.order_by(CommunicationLog.id.desc())  # type: ignore
        .offset(offset)
        .limit(limit)
    ).all()
