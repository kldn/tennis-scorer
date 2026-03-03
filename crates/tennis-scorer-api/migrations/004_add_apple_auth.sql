ALTER TABLE users
    ADD COLUMN apple_user_id TEXT UNIQUE,
    ALTER COLUMN email DROP NOT NULL,
    ALTER COLUMN password_hash DROP NOT NULL,
    ADD CONSTRAINT users_auth_method_chk
      CHECK (
        apple_user_id IS NOT NULL
        OR (email IS NOT NULL AND password_hash IS NOT NULL)
      );
