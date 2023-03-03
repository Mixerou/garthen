CREATE TABLE "devices"
(
    id            BIGINT PRIMARY KEY,
    external_id   SMALLINT,
    name          VARCHAR(24),
    status        SMALLINT  NOT NULL DEFAULT 1,
    kind          SMALLINT  NOT NULL,
    greenhouse_id BIGINT    NOT NULL
        CONSTRAINT devices_greenhouses_id_fk
            REFERENCES greenhouses
            ON UPDATE RESTRICT ON DELETE CASCADE,
    created_at    TIMESTAMP NOT NULL DEFAULT current_timestamp,
    UNIQUE (external_id, kind, greenhouse_id)
);

CREATE INDEX devices_greenhouse_id_index
    ON devices (greenhouse_id);

CREATE INDEX devices_created_at_index
    ON devices (created_at);

CREATE SEQUENCE snowflake_id_sequence minvalue 0 maxvalue 4096 cycle;

CREATE OR REPLACE FUNCTION generate_snowflake_id() RETURNS bigint AS
$$
DECLARE
    our_epoch   BIGINT := 1672531200000;
    sequence_id BIGINT;
    now_millis  BIGINT;
    machine_id  INT    := 0;
    node_id     INT    := 0;
BEGIN
    SELECT nextval('snowflake_id_sequence') % 4096 INTO sequence_id;
    SELECT floor(extract(EPOCH FROM clock_timestamp()) * 1000) INTO now_millis;

    return ((now_millis - our_epoch) << 22) | (machine_id << 17) | (node_id << 12) | sequence_id;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION insert_default_devices() RETURNS TRIGGER AS
$$
BEGIN
    INSERT INTO devices (id, external_id, kind, greenhouse_id)
        -- Temperature Sensors (Humidity)
    VALUES (generate_snowflake_id(), 1, 0, NEW.id),
           (generate_snowflake_id(), 2, 0, NEW.id),
           (generate_snowflake_id(), 3, 0, NEW.id),
           (generate_snowflake_id(), 4, 0, NEW.id),
           -- Soil Moisture Sensors
           (generate_snowflake_id(), 1, 1, NEW.id),
           (generate_snowflake_id(), 2, 1, NEW.id),
           (generate_snowflake_id(), 3, 1, NEW.id),
           (generate_snowflake_id(), 4, 1, NEW.id),
           (generate_snowflake_id(), 5, 1, NEW.id),
           (generate_snowflake_id(), 6, 1, NEW.id),
           -- Temperature Sensors (Temperature)
           (generate_snowflake_id(), 1, 2, NEW.id),
           (generate_snowflake_id(), 2, 2, NEW.id),
           (generate_snowflake_id(), 3, 2, NEW.id),
           (generate_snowflake_id(), 4, 2, NEW.id),
           -- Humidification Controller
           (generate_snowflake_id(), NULL, 3, NEW.id),
           -- Irrigation Controllers
           (generate_snowflake_id(), 1, 4, NEW.id),
           (generate_snowflake_id(), 2, 4, NEW.id),
           (generate_snowflake_id(), 3, 4, NEW.id),
           (generate_snowflake_id(), 4, 4, NEW.id),
           (generate_snowflake_id(), 5, 4, NEW.id),
           (generate_snowflake_id(), 6, 4, NEW.id),
           -- Windows Controller
           (generate_snowflake_id(), NULL, 5, NEW.id);

    return NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER greenhouses_insert
    AFTER INSERT
    ON greenhouses
    FOR EACH ROW
EXECUTE FUNCTION insert_default_devices();

DO
$$
    DECLARE
        greenhouse RECORD;
    BEGIN
        FOR greenhouse IN
            SELECT * FROM greenhouses
            LOOP
                INSERT INTO devices (id, external_id, kind, greenhouse_id)
                    -- Temperature Sensors (Humidity)
                VALUES (generate_snowflake_id(), 1, 0, greenhouse.id),
                       (generate_snowflake_id(), 2, 0, greenhouse.id),
                       (generate_snowflake_id(), 3, 0, greenhouse.id),
                       (generate_snowflake_id(), 4, 0, greenhouse.id),
                       -- Soil Moisture Sensors
                       (generate_snowflake_id(), 1, 1, greenhouse.id),
                       (generate_snowflake_id(), 2, 1, greenhouse.id),
                       (generate_snowflake_id(), 3, 1, greenhouse.id),
                       (generate_snowflake_id(), 4, 1, greenhouse.id),
                       (generate_snowflake_id(), 5, 1, greenhouse.id),
                       (generate_snowflake_id(), 6, 1, greenhouse.id),
                       -- Temperature Sensors (Temperature)
                       (generate_snowflake_id(), 1, 2, greenhouse.id),
                       (generate_snowflake_id(), 2, 2, greenhouse.id),
                       (generate_snowflake_id(), 3, 2, greenhouse.id),
                       (generate_snowflake_id(), 4, 2, greenhouse.id),
                       -- Humidification Controller
                       (generate_snowflake_id(), NULL, 3, greenhouse.id),
                       -- Irrigation Controllers
                       (generate_snowflake_id(), 1, 4, greenhouse.id),
                       (generate_snowflake_id(), 2, 4, greenhouse.id),
                       (generate_snowflake_id(), 3, 4, greenhouse.id),
                       (generate_snowflake_id(), 4, 4, greenhouse.id),
                       (generate_snowflake_id(), 5, 4, greenhouse.id),
                       (generate_snowflake_id(), 6, 4, greenhouse.id),
                       -- Windows Controller
                       (generate_snowflake_id(), NULL, 5, greenhouse.id);
            END LOOP;
    END
$$
