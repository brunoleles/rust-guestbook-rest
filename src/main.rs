use actix_web::{
    delete, get, middleware, post,
    web::{self, Json, Form},
    App, HttpServer, Responder,
};
use diesel::prelude::*;
use rust_guestbook_rest::module_guestbook::{guestbook_list, DeleteGuestbookRequest, PostGuestbookRequest};
// use rust_guestbook_rest::{
//     establish_connection, models::GuestbookModel, schema::guestbooks, GuestbookReposiory,
// };

#[get("/ping")]
async fn ping() -> impl Responder {
    format!("pong")
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/guestbook")]
async fn get_guestbook() -> impl Responder {
    let entries = guestbook_list();

    Json(entries)
}

#[post("/guestbook")]
async fn post_guestbook(request: Form<PostGuestbookRequest>) -> impl Responder {
    format!("post guest! n: {}, m: {}", request.name, request.message)
}

#[delete("/guestbook")]
async fn delete_guestbook(request: Form<DeleteGuestbookRequest>) -> impl Responder {
    format!("delete guest! {}", request.id)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            //
            .wrap(middleware::Logger::default())
            //
            .service(ping)
            .service(greet)
            //
            .service(get_guestbook)
            .service(post_guestbook)
            .service(delete_guestbook)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
