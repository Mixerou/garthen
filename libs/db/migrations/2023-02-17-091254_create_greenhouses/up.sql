CREATE TABLE "greenhouses"
(
    id         BIGINT PRIMARY KEY,
    name       VARCHAR(32) NOT NULL,
    token      VARCHAR(32) NOT NULL UNIQUE,
    owner_id   BIGINT      NOT NULL
        CONSTRAINT greenhouses_users_id_fk
            REFERENCES users
            ON UPDATE RESTRICT ON DELETE RESTRICT,
    created_at TIMESTAMP   NOT NULL DEFAULT current_timestamp
);
