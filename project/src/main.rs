use actix_web::{web, App, HttpServer};

use crate::controllers::{client_controller, driver_controller, events_controller};

mod controllers;
mod models;
mod routers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = client_controller::init_db()
        .await
        .expect("Failed to initialize database");

    println!("🚀 Server listening on http://127.0.0.1:3007");
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api")
                .service(client_controller::create_client)
                .service(client_controller::get_clients)
                .service(client_controller::get_client)
                .service(client_controller::update_client)
                .service(client_controller::delete_client)
                .service(driver_controller::create_driver)
                .service(driver_controller::get_drivers)
                .service(driver_controller::get_driver)
                .service(driver_controller::update_driver)
                .service(driver_controller::delete_driver)
                .service(events_controller::create_event)
                .service(events_controller::get_events)
                .service(events_controller::get_event)
                .service(events_controller::update_event)
                .service(events_controller::delete_event)
                .service(events_controller::get_events_by_driver)
                .service(events_controller::get_events_by_client)
            )
    })
    .bind(("127.0.0.1", 3007))?
    .run()
    .await
}
