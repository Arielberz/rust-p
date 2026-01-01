use actix_web::{get, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    "Hello, world!"
}

#[get("/api/persons")]
pub async fn get_all_persons() -> impl Responder {
    "persons ....."
}
