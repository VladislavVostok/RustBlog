use actix_web::{web, HttpResponse};
use actix_identity::Identity;
use tera::Context;
use crate::models::user::User;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthForm{
    username: String,
    password: String,
}

#[derive(Deserialize)]
pub struct RegisterForm{
    username: String,
    email: String,
    password: String,
    password_confirm: String,
}

pub async fn login(
    form: web::Form<AuthForm>,
    pool: web::Data<sqlx::MySqlPool>,
    identity: Identity,
) -> HttpResponse {

    let user = match User::find_by_username(&form.username, &pool).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::BadRequest().body("Неверный пароль или логин"),
    };

    if !user.verify_password(&form.password){
        return HttpResponse::BadRequest().body("Неверный пароль или логин");
    }

    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}

