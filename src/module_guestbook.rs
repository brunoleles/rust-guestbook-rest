use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::guestbooks;

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = guestbooks)]
pub struct GuestbookModel {
    pub id: i32,
    pub name: String,
    pub message: String,
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
