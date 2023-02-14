CREATE TABLE "users"
(
    id            BIGINT PRIMARY KEY,
    email         VARCHAR(512) NOT NULL UNIQUE,
    password_hash TEXT         NOT NULL,
    username      VARCHAR(32)  NOT NULL UNIQUE,
    created_at    TIMESTAMP    NOT NULL DEFAULT current_timestamp
);

ALTER TABLE sessions
    ADD CONSTRAINT sessions_users_id_fk
        FOREIGN KEY (user_id) REFERENCES users
            ON DELETE SET NULL;
