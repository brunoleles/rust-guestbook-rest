use diesel::prelude::*;

#[derive(Queryable)]
pub struct Guestbook {
    pub id: i32,    
    pub name: String,
    pub message: String,
}
