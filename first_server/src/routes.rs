use actix_web::web;
use crate::handlers;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        .service(handlers::hello)
        .service(handlers::get_all_persons);
}
