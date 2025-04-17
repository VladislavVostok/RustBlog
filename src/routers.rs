use actix_web::web;
use crate::controllers::post_controller;

pub fn configure(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("")
            .route("/", web::get().to(post_controller::index))
            .route("/posts/{id}", web::get().to(post_controller::show))
    );
}