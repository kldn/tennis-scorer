CREATE TABLE IF NOT EXISTS matches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    client_id UUID UNIQUE,
    match_type TEXT NOT NULL DEFAULT 'singles',
    config JSONB NOT NULL,
    winner SMALLINT NOT NULL,
    player1_sets SMALLINT NOT NULL,
    player2_sets SMALLINT NOT NULL,
    started_at TIMESTAMPTZ NOT NULL,
    ended_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_matches_user_id ON matches(user_id);
CREATE INDEX idx_matches_started_at ON matches(started_at);
