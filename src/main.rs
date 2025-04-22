mod routers;

use actix_identity::IdentityMiddleware;
use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
// use std::env;
use actix_web::{web, App, HttpServer};
// use dotenv::dotenv;

mod controllers;
mod models;
mod views;
mod middlewares;

#[actix_web::main]
async fn main() -> std::io::Result<()> {  //
    // dotenv().ok();

    // let database_ulr = env::var("DATABASE_URL")
    //                 .expect("DATABASE_URL должен быть установлен в переменных окружения ОС");

    let database_url = "mysql://admin:shalom***@127.0.0.1/blog_db".to_string();
    let secret_key = "jkadhgsut9[qu4t09843utiaghoihasdtqwtasgdgaw3taeuytq[4tuqieowjtgksdjg";
    let pool = sqlx::mysql::MySqlPool::connect(&database_url)
        .await
        .expect("Ошибка создания пула");

    let tera = views::init_template();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera.clone()))
            .wrap(IdentityMiddleware::default())
            // .wrap(
            //     SessionMiddleware::builder(
            //         CookieSessionStore::default(),
            //         secret_key.as_bytes().to_vec(),
            //     )
            // )
        .configure(routers::configure)

    }).bind(("127.0.0.1", 80))?
        .run()
        .await




}
