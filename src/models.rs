use diesel::deserialize::{self, FromSql};
#[macro_use]

use diesel::{prelude::*};
use diesel::pg::{Pg, PgConnection};
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use crate::schema::sql_types::StatusEnum;
use crate::schema::task;
use diesel::serialize::{self, IsNull, Output, ToSql};

use std::fmt;
use std::io::Write;


#[derive(Debug, AsExpression, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, FromSqlRow)]
#[diesel(sql_type = StatusEnum)]
pub enum Status {
    Pending,
    Completed,
}

#[derive(Queryable, Selectable, Serialize, Deserialize, Debug)]
#[diesel(table_name = task)]
#[diesel(check_for_backend(Pg))]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub status: Status,
    pub created_at: Option<chrono::NaiveDateTime>,
}
#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::task)]
pub struct NewTask {
    pub title: String,
    pub description: Option<String>,
    pub status: Status,
}


#[derive(Debug, Deserialize, Validate)]
pub struct CreateTaskDto {
    #[validate(length(min = 1, message = "Title must  not be empty"))]
    pub title: String,
    #[validate(length(min = 1, message = "Description must not be empty"))]
    pub description: Option<String>,
}


#[derive(Debug, Deserialize, Validate, AsChangeset)]
#[diesel(table_name = crate::schema::task)]
pub struct UpdateTaskDto {
    #[validate(length(min = 1, message = "Title must  not be empty"))]
    pub title: Option<String>,
    #[validate(length(min= 1, message = "Description must not be empty"))]
    pub description: Option<String>,
    pub status: Option<Status>,
}
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> Pool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}



impl FromSql<StatusEnum, Pg> for Status {
    fn from_sql(bytes: diesel::backend::RawValue<Pg>) -> deserialize::Result<Self> {
        match std::str::from_utf8(bytes.as_bytes())? {
            "pending" => Ok(Status::Pending),
            "completed" => Ok(Status::Completed),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl ToSql<StatusEnum, Pg> for Status {
    fn to_sql(&self, out: &mut Output<Pg>) -> serialize::Result {
        match *self {
            Status::Pending => out.write_all(b"pending")?,
            Status::Completed => out.write_all(b"completed")?,
        }
        Ok(IsNull::No)
    }
}