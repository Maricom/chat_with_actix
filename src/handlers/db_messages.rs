use actix::Message;
use actix_web::error;
use serde_derive::Deserialize;
use crate::models::Chat;

#[derive(Debug, Deserialize)]
pub struct User {
    pub user_name: String
}

impl Message for User {
    type Result = Result<Vec<Chat>, error::Error>;
}

#[derive(Debug, Deserialize)]
pub struct Transcript {
    pub user_name: String,
    pub body: String
}

impl Message for Transcript {
    type Result = Result<Chat, error::Error>;
}
