CREATE TABLE mints (
    id SERIAL PRIMARY KEY,
    p TEXT NOT NULL,
    op TEXT NOT NULL,
    tick TEXT NOT NULL,
    tick_id TEXT NOT NULL,
    amt TEXT NOT NULL,
    input_data TEXT NOT NULL,
    trx_hash TEXT NOT NULL,
    chain_id BIGINT NOT NULL,
    from_address TEXT NOT NULL,
    to_address TEXT NOT NULL,
    height BIGINT NOT NULL,
    timestamp BIGINT NOT NULL,
    UNIQUE (chain_id, tick, tick_id)
);