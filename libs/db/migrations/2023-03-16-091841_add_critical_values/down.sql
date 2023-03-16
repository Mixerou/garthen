ALTER TABLE greenhouses
    DROP COLUMN maximum_average_humidity;

ALTER TABLE greenhouses
    DROP COLUMN minimum_average_temperature;

ALTER TABLE devices
    DROP COLUMN maximum_data_value;

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

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
