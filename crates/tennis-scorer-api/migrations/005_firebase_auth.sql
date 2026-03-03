-- Replace Apple auth + email/password auth with Firebase Auth.
-- firebase_uid is the unified identity across all providers.

ALTER TABLE users
    DROP CONSTRAINT IF EXISTS users_auth_method_chk,
    ADD COLUMN IF NOT EXISTS firebase_uid TEXT NOT NULL UNIQUE,
    ADD COLUMN IF NOT EXISTS display_name TEXT,
    ADD COLUMN IF NOT EXISTS avatar_url TEXT,
    DROP COLUMN IF EXISTS password_hash,
    DROP COLUMN IF EXISTS apple_user_id;
