use actix_web::{web, Scope};
use crate::controllers::client_controller;

pub fn client_routes() -> Scope {
    web::scope("")
        .service(client_controller::create_client)
        .service(client_controller::get_clients)
        .service(client_controller::get_client)
        .service(client_controller::update_client)
        .service(client_controller::delete_client)
}
