DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'public'
          AND table_name = 'users'
          AND column_name = 'apple_user_id'
    ) THEN
        ALTER TABLE users ADD COLUMN apple_user_id TEXT UNIQUE;
    END IF;
END $$;

ALTER TABLE users ALTER COLUMN email DROP NOT NULL;
ALTER TABLE users ALTER COLUMN password_hash DROP NOT NULL;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_constraint c
        JOIN pg_class t ON t.oid = c.conrelid
        JOIN pg_namespace n ON n.oid = t.relnamespace
        WHERE c.conname = 'users_auth_method_chk'
          AND t.relname = 'users'
          AND n.nspname = 'public'
    ) THEN
        ALTER TABLE users ADD CONSTRAINT users_auth_method_chk
          CHECK (
            apple_user_id IS NOT NULL
            OR (email IS NOT NULL AND password_hash IS NOT NULL)
          );
    END IF;
END $$;
