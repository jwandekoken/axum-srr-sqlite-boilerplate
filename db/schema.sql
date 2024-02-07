CREATE TABLE IF NOT EXISTS "schema_migrations" (version varchar(128) primary key);
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    email VARCHAR NOT NULL UNIQUE,
    hashed_password VARCHAR NOT NULL,
    reset_password_selector VARCHAR,
    reset_password_sent_at TIMESTAMP,
    reset_password_validator_hash VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE sessions (
    id INTEGER PRIMARY KEY,
    -- The session is a 32 byte random number stored in their cookie. This is the sha256 hash of that value.
    session_verifier VARCHAR NOT NULL,
    user_id INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- A 6 digit code that is encrypted here to prevent attackers with read access to the database being able to use it.
    otp_code_encrypted VARCHAR NOT NULL,
    -- We count OTP attempts to prevent brute forcing.
    otp_code_attempts INTEGER NOT NULL DEFAULT 0,
    -- Once the user enters the correct value this gets set to true.
    otp_code_confirmed BOOLEAN NOT NULL DEFAULT false,
    -- Have we sent the OTP code?
    otp_code_sent BOOLEAN NOT NULL DEFAULT false
);
-- Dbmate schema migrations
INSERT INTO "schema_migrations" (version) VALUES
  ('20240207111329');
