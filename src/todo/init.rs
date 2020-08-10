use actix_web::{web};
use crate::todo::api;
pub fn initialize(cfg: &mut web::ServiceConfig) {
    cfg.service(api::list)
        .service(api::create)
        .service(api::update)
        .service(api::find)
    ;
}