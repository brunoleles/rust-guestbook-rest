use actix_web::{get, web, App, HttpServer, Responder, middleware, post, delete};
use diesel::prelude::*;
use rust_guestbook_rest::{establish_connection, models::Guestbook, schema::guestbooks};

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
    format!("list guests!")
}

#[post("/guestbook")]
async fn post_guestbook() -> impl Responder {
    format!("post guest!")
}

#[delete("/guestbook")]
async fn delete_guestbook() -> impl Responder {
    format!("delete guest!")
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

    // use self::guestbooks::dsl::*;

    // let connection = &mut establish_connection();
    // let results = guestbooks
    //     //.filter(published.eq(true))
    //     .limit(5)
    //     .load::<Guestbook>(connection)
    //     .expect("Error loading posts");

    // println!("Displaying {} posts", results.len());
    // for entry in results {
    //     println!("{}", entry.name);
    //     println!("{}", entry.message);
    //     println!("----------\n");
    // }
}
