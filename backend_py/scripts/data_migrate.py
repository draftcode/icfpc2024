from backend_rs import evaluate_message  # type: ignore
from backend_py.db import engine

from sqlmodel import Session, select
from backend_py.models import CommunicationLog

with Session(engine) as session:
    logs = session.exec(select(CommunicationLog))
    for log in logs:
        if log.decoded_response is None:
            continue
        if log.decoded_response.startswith("Correct, you solved lambdaman"):
            req = evaluate_message(log.request)
            log.decoded_request = req
            log.decoded_request_prefix = req[:100]
    session.commit()
