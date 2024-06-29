import datetime

import httpx
from fastapi import HTTPException

from backend_rs import decode_message  # type: ignore

from .config import settings
from .deps import SessionDep
from .models import CommunicationLog

http_client = httpx.Client(headers={"Authorization": f"Bearer {settings.API_TOKEN}"})


def send_encoded_req(session: SessionDep, encoded_req: str) -> CommunicationLog:
    resp = http_client.post(
        "https://boundvariable.space/communicate", content=encoded_req
    )
    if not resp.is_success:
        raise HTTPException(status_code=resp.status_code, detail=resp.text)

    encoded_resp = resp.text
    decoded_resp = decode_message(encoded_resp)
    decoded_req = decode_message(encoded_req)
    log = CommunicationLog(
        created=datetime.datetime.now(),
        request=encoded_req,
        response=encoded_resp,
        decoded_request_prefix=decoded_req[:100],
        decoded_request=decoded_req,
        decoded_response=decoded_resp,
    )
    session.add(log)
    session.commit()
    # Need to touch something to get back the data?
    log.id
    return log
