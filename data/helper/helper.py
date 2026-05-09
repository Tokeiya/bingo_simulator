import psycopg
from psycopg.connection import Connection
from psycopg import sql


def connect_to_dell() -> Connection:
    return psycopg.connect("host=dell1 port=5432 dbname=bingo user=tokeiya3")


def create_partitioned_table():
    with psycopg.connect("host=dell1 port=5432 dbname=bingo user=tokeiya3") as conn:
        with conn.transaction() as trn:
            with conn.cursor() as cur:
                _ = cur.execute("DROP TABLE IF EXISTS all_cards CASCADE ")
                _ = cur.execute("""CREATE TABLE all_cards
                               (
                                   id        INTEGER GENERATED ALWAYS AS IDENTITY,
                                   play_id   INTEGER NOT NULL,
                                   cards     INTEGER NOT NULL,
                                   round     INTEGER NOT NULL,
                                   hit_count INTEGER NOT NULL,
                                   PRIMARY KEY (id, cards)
                               ) PARTITION BY LIST (cards) """)

                for i in [x * 5 for x in range(1, 21)]:
                    _ = cmd = sql.SQL(
                        "CREATE UNLOGGED TABLE {} PARTITION OF all_cards FOR VALUES IN ({})"
                    ).format(
                        sql.Identifier(f"cards_{i}"),
                        sql.Literal(i),
                    )
                    print(cmd.as_string(conn))
                    _ = cur.execute(cmd)

                _ = cur.execute(
                    "CREATE UNLOGGED TABLE cards_default PARTITION OF all_cards DEFAULT"
                )


def create_index():
    with connect_to_dell() as conn:
        with conn.transaction() as trn:
            with conn.cursor() as cur:
                _ = cur.execute("CREATE INDEX ON all_cards USING btree (play_id)")
                _ = cur.execute("CREATE INDEX ON all_cards USING btree (round)")
                _ = cur.execute("CREATE INDEX ON all_cards USING btree (hit_count)")
                _ = cur.execute("CREATE INDEX ON all_cards USING btree (cards)")
