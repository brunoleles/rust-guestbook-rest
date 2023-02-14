use actix_web::{
    delete, get, middleware, post,
    web::{Form, Json},
    App, HttpServer, Responder,
};
use diesel::prelude::*;
use rust_guestbook_rest::{establish_connection, module_guestbook::*};

#[get("/ping")]
async fn ping() -> impl Responder {
    format!("pong")
}

#[get("/guestbook")]
async fn api_list_guestbooks() -> impl Responder {
    use rust_guestbook_rest::schema::guestbooks::dsl::*;

    let connection = &mut establish_connection();
    let results = guestbooks
        //.filter(published.eq(true))
        .limit(5)
        .load::<GuestbookModel>(connection)
        .expect("Error loading posts");

    Json(results)
}

#[post("/guestbook")]
async fn api_post_guestbook(request: Form<PostGuestbookRequest>) -> impl Responder {
    format!("post guest! n: {}, m: {}", request.name, request.message)
}

#[delete("/guestbook")]
async fn api_delete_guestbook(request: Form<DeleteGuestbookRequest>) -> impl Responder {
    format!("delete guest! {}", request.id)

    // let connection = &mut establish_connection();
    // let result = diesel::delete(guestbooks.filter(id.eq(request.id)))
    //     .execute(connection)
    //     .expect("Unable to delete record");

    // result > 0
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            //
            .wrap(middleware::Logger::default())
            //
            .service(ping)
            //
            .service(api_list_guestbooks)
            .service(api_post_guestbook)
            .service(api_delete_guestbook)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
