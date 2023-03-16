ALTER TABLE greenhouses
    ADD maximum_average_humidity DOUBLE PRECISION DEFAULT 80.0;

ALTER TABLE greenhouses
    ADD minimum_average_temperature DOUBLE PRECISION DEFAULT 21.0;

ALTER TABLE devices
    ADD maximum_data_value DOUBLE PRECISION;

UPDATE devices
SET maximum_data_value = 80.0
WHERE kind = 1;

CREATE OR REPLACE FUNCTION insert_default_devices() RETURNS TRIGGER AS
$$
BEGIN
    INSERT INTO devices (id, external_id, kind, greenhouse_id)
        -- Temperature Sensors (Humidity)
    VALUES (generate_snowflake_id(), 1, 0, NEW.id),
           (generate_snowflake_id(), 2, 0, NEW.id),
           (generate_snowflake_id(), 3, 0, NEW.id),
           (generate_snowflake_id(), 4, 0, NEW.id),
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

    INSERT INTO devices (id, external_id, kind, greenhouse_id, maximum_data_value)
        -- Soil Moisture Sensors
    VALUES (generate_snowflake_id(), 1, 1, NEW.id, 80.0),
           (generate_snowflake_id(), 2, 1, NEW.id, 80.0),
           (generate_snowflake_id(), 3, 1, NEW.id, 80.0),
           (generate_snowflake_id(), 4, 1, NEW.id, 80.0),
           (generate_snowflake_id(), 5, 1, NEW.id, 80.0),
           (generate_snowflake_id(), 6, 1, NEW.id, 80.0);

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
