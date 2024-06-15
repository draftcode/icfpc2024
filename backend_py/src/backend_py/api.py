import importlib.resources as pkg_resources

from backend_rs import hello  # type: ignore
from fastapi import FastAPI
from fastapi.responses import RedirectResponse
from pydantic import BaseModel
from sqlmodel import select
from typing import Sequence

from . import models
from .deps import SessionDep

app = FastAPI()


@app.get("/", include_in_schema=False)
async def root():
    return RedirectResponse("/docs")


class SampleResponse(BaseModel):
    backend_rs: str
    data_test_txt: str
    dummies: Sequence[models.DummyProblem]


@app.get("/message")
async def message(session: SessionDep) -> SampleResponse:
    dummies = session.exec(select(models.DummyProblem)).all()
    content = pkg_resources.files("backend_py").joinpath("data/test.txt").read_text()
    return SampleResponse(
        backend_rs=hello(),
        data_test_txt=content,
        dummies=dummies,
    )
