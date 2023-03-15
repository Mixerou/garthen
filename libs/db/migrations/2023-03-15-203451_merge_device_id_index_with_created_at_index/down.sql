CREATE INDEX device_records_device_id_index
    ON device_records (device_id);

CREATE INDEX device_records_created_at_index
    ON device_records (created_at);

DROP INDEX device_records_device_id_created_at_index;
