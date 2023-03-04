use std::mem::transmute;
use std::time::SystemTime;

use db::schema::devices;
use diesel::{deserialize, ExpressionMethods, Insertable, Queryable, RunQueryDsl};
use diesel::deserialize::FromStaticSqlRow;
use diesel::expression::AsExpression;
use diesel::helper_types::AsExprOf;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::row::Row;
use diesel::sql_types::SmallInt;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::error::WebSocketError;

#[derive(Clone, Debug, Deserialize, Serialize, Insertable, Queryable)]
#[diesel(table_name = devices)]
pub struct Device {
    pub id: i64,
    pub external_id: Option<i16>,
    pub name: Option<String>,
    pub status: DeviceStatus,
    pub kind: DeviceKind,
    pub greenhouse_id: i64,
    pub created_at: SystemTime,
}

impl Device {
    pub fn find(id: i64) -> Result<Self, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let device = devices::table
            .filter(devices::id.eq(id))
            .first(connection)?;

        Ok(device)
    }

    pub fn find_all_by_greenhouse_id(greenhouse_id: i64) -> Result<Vec<Self>, WebSocketError> {
        let connection = &mut db::get_connection()?;

        let devices = devices::table
            .filter(devices::greenhouse_id.eq(greenhouse_id))
            .load(connection)?;

        Ok(devices)
    }
}

#[derive(Copy, Clone, Debug, Deserialize_repr, Serialize_repr, Eq, PartialEq)]
#[repr(i16)]
pub enum DeviceStatus {
    Offline = 0,
    Online = 1,
    Disabled = 2,
}

impl FromStaticSqlRow<SmallInt, Pg> for DeviceStatus {
    fn build_from_row<'a>(row: &impl Row<'a, Pg>) -> deserialize::Result<Self> {
        Ok(unsafe { transmute(i16::build_from_row(row)?) })
    }
}

impl AsExpression<SmallInt> for DeviceStatus {
    type Expression = AsExprOf<i16, SmallInt>;

    fn as_expression(self) -> Self::Expression {
        <i16 as AsExpression<SmallInt>>::as_expression(self as i16)
    }
}

impl<'a> AsExpression<SmallInt> for &'a DeviceStatus {
    type Expression = AsExprOf<i16, SmallInt>;

    fn as_expression(self) -> Self::Expression {
        <i16 as AsExpression<SmallInt>>::as_expression(*self as i16)
    }
}

impl Queryable<SmallInt, Pg> for DeviceStatus {
    type Row = i16;

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(unsafe { transmute(row) })
    }
}

#[derive(Copy, Clone, Debug, Deserialize_repr, Serialize_repr, Eq, PartialEq)]
#[repr(i16)]
pub enum DeviceKind {
    HumiditySensor = 0,
    SoilMoistureSensor = 1,
    TemperatureSensor = 2,
    HumidificationController = 3,
    IrrigationControllers = 4,
    WindowsController = 5,
}

impl FromStaticSqlRow<SmallInt, Pg> for DeviceKind {
    fn build_from_row<'a>(row: &impl Row<'a, Pg>) -> deserialize::Result<Self> {
        Ok(unsafe { transmute(i16::build_from_row(row)?) })
    }
}

impl AsExpression<SmallInt> for DeviceKind {
    type Expression = AsExprOf<i16, SmallInt>;

    fn as_expression(self) -> Self::Expression {
        <i16 as AsExpression<SmallInt>>::as_expression(self as i16)
    }
}

impl<'a> AsExpression<SmallInt> for &'a DeviceKind {
    type Expression = AsExprOf<i16, SmallInt>;

    fn as_expression(self) -> Self::Expression {
        <i16 as AsExpression<SmallInt>>::as_expression(*self as i16)
    }
}

impl Queryable<SmallInt, Pg> for DeviceKind {
    type Row = i16;

    fn build(row: Self::Row) -> deserialize::Result<Self> {
        Ok(unsafe { transmute(row) })
    }
}
