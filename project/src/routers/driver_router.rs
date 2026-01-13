use actix_web::{web, Scope};
use crate::controllers::driver_controller;

pub fn driver_routes() -> Scope {
    web::scope("")
        .service(driver_controller::create_driver)
        .service(driver_controller::get_drivers)
        .service(driver_controller::get_driver)
        .service(driver_controller::update_driver)
        .service(driver_controller::delete_driver)
}
