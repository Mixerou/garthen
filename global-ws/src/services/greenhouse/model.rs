use std::time::SystemTime;

use db::schema::greenhouses;
use diesel::{ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::error::{WebSocketError, WebSocketErrorTemplate};

#[derive(Clone, Debug, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = greenhouses)]
pub struct Greenhouse {
    pub id: i64,
    pub name: String,
    pub token: String,
    pub owner_id: i64,
    pub created_at: SystemTime,
    pub maximum_average_humidity: Option<f64>,
    pub minimum_average_temperature: Option<f64>,
}

impl Greenhouse {
    // CRUD
    pub fn create(greenhouse: NewGreenhouse) -> Result<Self, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let greenhouse = Greenhouse {
            id: snowflake::generate(),
            name: greenhouse.name,
            token: greenhouse.token,
            owner_id: greenhouse.owner_id,
            created_at: SystemTime::now(),
            maximum_average_humidity: Some(80.0),
            minimum_average_temperature: Some(21.0),
        };

        let session = diesel::insert_into(greenhouses::table)
            .values(greenhouse)
            .get_result(connection)?;

        Ok(session)
    }

    pub fn find(id: i64) -> Result<Self, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let greenhouse = greenhouses::table
            .filter(greenhouses::id.eq(id))
            .first(connection)?;

        Ok(greenhouse)
    }

    pub fn find_by_id_and_owner_id(id: i64, owner_id: i64) -> Result<Self, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let greenhouse = greenhouses::table
            .filter(greenhouses::id.eq(id))
            .filter(greenhouses::owner_id.eq(owner_id))
            .first(connection)?;

        Ok(greenhouse)
    }

    pub fn find_by_token(token: String) -> Result<Self, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let greenhouse = greenhouses::table
            .filter(greenhouses::token.eq(token))
            .first(connection)?;

        Ok(greenhouse)
    }

    pub fn find_all_by_owner_id(owner_id: i64) -> Result<Vec<Self>, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let greenhouses = greenhouses::table
            .filter(greenhouses::owner_id.eq(owner_id))
            .load(connection)?;

        Ok(greenhouses)
    }

    pub fn count_by_owner_id(owner_id: i64) -> Result<i64, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let greenhouses = greenhouses::table
            .filter(greenhouses::owner_id.eq(owner_id))
            .count()
            .get_result(connection)?;

        Ok(greenhouses)
    }

    pub fn update(
        id: i64,
        new_name: String,
        new_token: String,
        new_maximum_average_humidity: Option<f64>,
        new_minimum_average_temperature: Option<f64>,
    ) -> Result<Self, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let greenhouse = diesel::update(greenhouses::table)
            .filter(greenhouses::id.eq(id))
            .set((
                greenhouses::name.eq(new_name),
                greenhouses::token.eq(new_token),
                greenhouses::maximum_average_humidity.eq(new_maximum_average_humidity),
                greenhouses::minimum_average_temperature.eq(new_minimum_average_temperature),
            ))
            .get_result(connection)?;

        Ok(greenhouse)
    }

    pub fn delete(id: i64) -> Result<usize, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let result = diesel::delete(
            greenhouses::table.filter(greenhouses::id.eq(id))
        ).execute(connection)?;

        Ok(result)
    }

    // Default implementations
    pub fn check_name_length(name: &str) -> Result<(), WebSocketError> {
        let name_length = name.chars().count();

        match name_length {
            length if length < 3 => Err(WebSocketErrorTemplate::GreenhouseNameTooShort(None).into()),
            length if length > 32 => Err(WebSocketErrorTemplate::GreenhouseNameTooLong(None).into()),
            _ => Ok(())
        }
    }

    pub fn check_token_length(token: &str) -> Result<(), WebSocketError> {
        let token_length = token.chars().count();

        match token_length {
            length if length < 3 => Err(WebSocketErrorTemplate::GreenhouseTokenTooShort(None).into()),
            length if length > 32 => Err(WebSocketErrorTemplate::GreenhouseTokenTooLong(None).into()),
            _ => Ok(())
        }
    }
}

pub struct NewGreenhouse {
    pub name: String,
    pub token: String,
    pub owner_id: i64,
}
