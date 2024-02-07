-- migrate:up
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

INSERT INTO users(email, hashed_password) VALUES('test1@test1.com', 'aasdsaddasad');
INSERT INTO users(email, hashed_password) VALUES('test2@test1.com', 'aasdsaddasad');
INSERT INTO users(email, hashed_password) VALUES('test3@test1.com', 'aasdsaddasad');

-- The users login sessions
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

-- migrate:down
DROP TABLE users;
DROP TABLE sessions;
