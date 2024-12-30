// @generated automatically by Diesel CLI.

pub mod sql_types {
    // #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    // #[diesel(postgres_type(name = "status_enum"))]

    #[derive(diesel::sql_types::SqlType, Debug)]
    #[diesel(postgres_type(name = "status_enum"))]
    pub struct StatusEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::StatusEnum;

    task (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        status -> StatusEnum,
        created_at -> Nullable<Timestamp>,
    }
}
