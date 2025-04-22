use std::convert::identity;
use actix_web::{dev::Payload, Error, FromRequest, HttpRequest, http::header, HttpMessage};
use actix_identity::Identity;
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::models::user::User;


#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticatedUser{
    pub user_id:u64,
    pub username: String,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(identity) = Identity::from_request(req, &mut Payload::None).into_inner().ok(){
            if let Ok(user_json) = identity.id() {
                if let Ok(user) = serde_json::from_str::<AuthenticatedUser>(&user_json){
                    return ready(Ok(user));
                }
            }
        }

        ready(Err(actix_web::error::ErrorUnauthorized("Трубуется авторизация")))
    }
}