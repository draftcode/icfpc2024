"""empty message

Revision ID: 09fbd809b8a2
Revises: 5bd8e1feeed0
Create Date: 2024-06-28 10:34:10.650887

"""
from alembic import op
import sqlalchemy as sa
import sqlmodel.sql.sqltypes


# revision identifiers, used by Alembic.
revision = '09fbd809b8a2'
down_revision = '5bd8e1feeed0'
branch_labels = None
depends_on = None


def upgrade():
    # ### commands auto generated by Alembic - please adjust! ###
    op.add_column('communicationlog', sa.Column('decoded_request', sqlmodel.sql.sqltypes.AutoString(), nullable=True))
    op.add_column('communicationlog', sa.Column('decoded_response', sqlmodel.sql.sqltypes.AutoString(), nullable=True))
    # ### end Alembic commands ###


def downgrade():
    # ### commands auto generated by Alembic - please adjust! ###
    op.drop_column('communicationlog', 'decoded_response')
    op.drop_column('communicationlog', 'decoded_request')
    # ### end Alembic commands ###