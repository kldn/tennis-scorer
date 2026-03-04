-- Replace Apple auth + email/password auth with Firebase Auth.
-- firebase_uid is the unified identity across all providers.

-- Step 1: structural changes (nullable firebase_uid initially)
ALTER TABLE users
    DROP CONSTRAINT IF EXISTS users_auth_method_chk,
    ADD COLUMN IF NOT EXISTS firebase_uid TEXT,
    ADD COLUMN IF NOT EXISTS display_name TEXT,
    ADD COLUMN IF NOT EXISTS avatar_url TEXT,
    DROP COLUMN IF EXISTS password_hash,
    DROP COLUMN IF EXISTS apple_user_id;

-- Step 2: backfill existing rows with a unique placeholder
UPDATE users SET firebase_uid = 'migrated_' || id::text WHERE firebase_uid IS NULL;

-- Step 3: enforce NOT NULL + UNIQUE
ALTER TABLE users ALTER COLUMN firebase_uid SET NOT NULL;
ALTER TABLE users ADD CONSTRAINT users_firebase_uid_unique UNIQUE (firebase_uid);
