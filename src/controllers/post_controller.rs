use actix_web::{web, HttpResponse};
use tera::Context;
use crate::models::post::Post;

pub async fn index(
    pool: web::Data<sqlx::MySqlPool>,
    tera: web::Data<tera::Tera>
) -> HttpResponse{

    let posts = Post::all(&pool).await.unwrap_or_default();
    let mut context = Context::new();
    context.insert("posts", &posts);

    let rendered = tera.render("index.html", &context).unwrap();

    HttpResponse::Ok().body(rendered)
}


pub async fn show(
    pool: web::Data<sqlx::MySqlPool>,
    tera: web::Data<tera::Tera>,
    path: web::Path<u64>
) -> HttpResponse {

    let post_id = path.into_inner();
    let post = Post::find(post_id, &pool).await.unwrap();

    let mut context = Context::new();
    context.insert("post", &post);

    let rendered = tera.render("post.html", &context).unwrap();

    HttpResponse::Ok().body(rendered)
}