from backend_rs import decode_message  # type: ignore
from backend_py.db import engine

from sqlmodel import Session, select
from backend_py.models import CommunicationLog

with Session(engine) as session:
    logs = session.exec(select(CommunicationLog))
    for log in logs:
        if log.decoded_request is None:
            log.decoded_request = decode_message(log.request)
        if log.decoded_response is None:
            log.decoded_response = decode_message(log.response)
        if log.decoded_request_prefix is None and log.decoded_request is not None:
            log.decoded_request_prefix = log.decoded_request[:100]
    session.commit()
