use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct GuestbookModel {
    pub id: i32,
    pub name: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct PostGuestbookRequest {
    pub name: String,
    pub message: String,
}

#[derive(Deserialize)]
pub struct DeleteGuestbookRequest {
    pub id: i32,
}
