import httpx
from backend_rs import decode_message  # type: ignore
from fastapi import FastAPI, Body
from fastapi.responses import RedirectResponse, PlainTextResponse
from .config import settings
from .deps import SessionDep
from .models import CommunicationLog
import datetime

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
