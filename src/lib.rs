pub mod module_guestbook;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use schema::guestbooks::message;
use serde::Serialize;
use std::{any, env};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Serialize)]
pub struct ApiOk {
    pub success: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,
}

#[derive(Serialize)]
pub struct ApiOkWithData<T> {
    pub success: bool,
    pub data: T,
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

impl<T> ApiOkWithData<T> {
    pub fn new(data: T) -> Self {
        Self {
            success: true,
            data: data,
            message: String::from(""),
        }
    }
}
