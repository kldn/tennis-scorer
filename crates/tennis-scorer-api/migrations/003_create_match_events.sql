CREATE TABLE IF NOT EXISTS match_events (
    id BIGSERIAL PRIMARY KEY,
    match_id UUID NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    point_number INT NOT NULL,
    player SMALLINT NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL,
    UNIQUE(match_id, point_number)
);

CREATE INDEX idx_match_events_match_id ON match_events(match_id);
