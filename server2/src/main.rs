// Import Actix-web components: #[get] macro, App, HttpServer, and the Responder trait
use actix_web::{get, App, HttpServer,Responder};

// Define a handler function for GET requests at the root path "/"
// #[get("/")] - Actix macro that maps this function to the "/" route
// The function is async and returns something that implements Responder

#[get("/")]
async fn hello() -> impl Responder {
    // The function returns a string that implements Responder trait
    "Hello, world!"
}
#[get("/api/persons")]
async fn get_all_persons() -> impl Responder {
    // The function returns a string that implements Responder trait
    "persons .....!"
}

#[actix_web::main]
async fn main() ->std::io::Result<()> {
    HttpServer::new(|| App::new()
    .service(hello)
    .service(get_all_persons))
        .bind("127.0.0.1:3000")?
        .run()
        .await
}