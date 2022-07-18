CREATE TABLE tracked_data (
    id               SERIAL PRIMARY KEY,
    arrived_at       TIMESTAMP NOT NULL,
    source_machine   TEXT NOT NULL,
    event_type       INTEGER NOT NULL,
    validation_ok    BOOLEAN NOT NULL,
    validation_error TEXT,
    found_fields     TEXT,
    raw_json         TEXT,
    raw_xml          TEXT NOT NULL
)