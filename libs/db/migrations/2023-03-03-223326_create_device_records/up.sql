CREATE TABLE "device_records"
(
    id         BIGINT PRIMARY KEY,
    device_id  BIGINT    NOT NULL
        CONSTRAINT device_records_devices_id_fk
            REFERENCES devices
            ON UPDATE RESTRICT ON DELETE CASCADE,
    data       FLOAT     NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT current_timestamp
);

CREATE INDEX device_records_device_id_index
    ON device_records (device_id);

CREATE INDEX device_records_created_at_index
    ON device_records (created_at);
