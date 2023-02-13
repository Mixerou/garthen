CREATE TABLE "sessions"
(
    id         BIGINT PRIMARY KEY,
    token      VARCHAR(96) NOT NULL UNIQUE,
    user_id    BIGINT,
    created_at TIMESTAMP   NOT NULL DEFAULT current_timestamp
);

CREATE INDEX sessions_user_id_index
    ON sessions (user_id);
