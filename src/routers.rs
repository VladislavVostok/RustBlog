use actix_web::web;
use crate::controllers::{post_controller, auth_controller};


pub fn configure(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("")
            .route("/", web::get().to(post_controller::index))
            .route("/posts/{id}", web::get().to(post_controller::show))


            .route("/login",  web::get().to(auth_controller::show_login))
            .route("/login",  web::post().to(auth_controller::login))
            .route("/register",  web::get().to(auth_controller::show_register))
            // .route("/register",  web::post().to(auth_controller::register))
    );
}