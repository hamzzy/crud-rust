#[macro_use]

use diesel::deserialize::{FromSql};
use diesel::{prelude::*};
use diesel::pg::{Pg, PgConnection};
use diesel::r2d2::{self, ConnectionManager};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};
use crate::schema::sql_types::StatusEnum;
use crate::schema::task;
use validator_derive::Validate;

use chrono::NaiveDateTime;




#[derive(Debug, Serialize, Deserialize, Clone, Copy, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::models::StatusEnum"]
pub enum Status {
    Pending,
    Completed,
}

#[derive(Queryable, Selectable, Serialize, Deserialize,Debug)]
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
    #[validate(length(min = 1, message = "Title must be between 1 and 100 characters"))]
    pub title: String,
    #[validate(length(min = 1, message = "Description cannot exceed 500 characters"))]
    pub description: Option<String>,
}


#[derive(Debug, Deserialize, Validate, AsChangeset)]
#[diesel(table_name = crate::schema::task)]
pub struct UpdateTaskDto {
    #[validate(length(min = 1, max = 100, message = "Title must be between 1 and 100 characters"))]
    pub title: Option<String>,
    #[validate(length(max = 500, message = "Description cannot exceed 500 characters"))]
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
