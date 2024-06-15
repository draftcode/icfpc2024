from sqlmodel import Field, SQLModel


class DummyProblem(SQLModel, table=True):
    id: int = Field(primary_key=True)
    name: str
