use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
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
    request: HttpRequest,
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

    let auth_user = serde_json::json!({
        "user_id": user.id,
        "username": user.username,
    }).to_string();

    Identity::login(&request.extensions(), auth_user.into()).unwrap();

    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}

pub async fn show_login(
    tera: web::Data<tera::Tera>
) -> HttpResponse {
    let rendered = tera.render("login.html", &Context::new()).unwrap();

    HttpResponse::Ok().body(rendered)
}

pub async fn show_register(
    tera: web::Data<tera::Tera>
) -> HttpResponse {
    let rendered = tera.render("register.html", &Context::new()).unwrap();

    HttpResponse::Ok().body(rendered)
}
