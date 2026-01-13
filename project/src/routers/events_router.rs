use actix_web::{web, Scope};
use crate::controllers::events_controller;

pub fn events_routes() -> Scope {
    web::scope("")
        .service(events_controller::create_event)
        .service(events_controller::get_events)
        .service(events_controller::get_event)
        .service(events_controller::update_event)
        .service(events_controller::delete_event)
        .service(events_controller::get_events_by_driver)
        .service(events_controller::get_events_by_client)
}
