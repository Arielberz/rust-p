// Import Actix-web components: #[get] macro, App, HttpServer, and the Responder trait
use actix_web::{get, App, HttpServer, Responder};
// Serde library is used for serialization (converting to JSON, XML, etc.)
use serde::Serialize;

// Define a struct that we want to return in the response
// #[derive(Serialize)] allows Actix/Serde to automatically serialize Item into JSON
#[derive(Serialize)]
struct Item {
    id: i32,
    name: String,
}

// Service function that handles GET requests to the "/item" path
// #[get("/item")] - Actix macro that maps this function to the route
// The function is async and returns something that implements Responder (e.g. HttpResponse)
#[get("/item")]
async fn get_item() -> impl Responder {
    // Create an Item instance
    let item = Item { id: 1, name: "Book".to_string() };
    // Return an HTTP 200 (OK) response with JSON body generated from the struct
    actix_web::HttpResponse::Ok().json(item)
}

// Main function of the server
// #[actix_web::main] - macro that sets up the async runtime (Tokio) for Actix
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create a new HTTP server
    HttpServer::new(|| {
        // Create a new Actix App and register the service (get_item) we defined
        App::new().service(get_item)
    })
    // Bind the server to local address 127.0.0.1 on port 8080
    .bind(("127.0.0.1", 8080))?
    // Run the server
    .run()
    .await
}