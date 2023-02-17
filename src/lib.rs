pub mod schema;
use crate::schema::guestbooks;
use diesel::prelude::*;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn transform_collection<F, T>(values: Vec<F>, mapper: fn(item: &F) -> T) -> Vec<T> {
    values.iter().map(|item| mapper(item)).collect()
}

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = guestbooks)]
pub struct GuestbookModel {
    pub id: i32,
    pub name: String,
    pub message: String,
    //pub created_at: chrono::NaiveDateTime, // TODO: adicionar field created_at
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = guestbooks)]
pub struct PostGuestbookRequest {
    pub name: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct DeleteGuestbookRequest {
    pub id: i32,
}

#[derive(Serialize)]
pub struct GuestbookApi {
    pub id: i32,
    pub name: String,
    pub message: String,
}

impl From<&GuestbookModel> for GuestbookApi {
    fn from(value: &GuestbookModel) -> Self {
        GuestbookApi {
            id: value.id,
            name: String::from(&value.name),
            message: String::from(&value.message),
        }
    }
}

#[derive(Serialize)]
pub struct ApiOk {
    pub success: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,
}

impl ApiOk {
    pub fn new() -> Self {
        ApiOk {
            success: true,
            message: String::from(""),
        }
    }
}

#[derive(Serialize)]
pub struct ApiOkWithData<T> {
    pub success: bool,
    pub data: T,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,
}

impl<T> ApiOkWithData<T> {
    pub fn new(data: T) -> Self {
        Self {
            success: true,
            data: data,
            message: String::from(""),
        }
    }
}
