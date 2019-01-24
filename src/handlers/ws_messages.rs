use actix::{Message, Recipient};
use uuid::Uuid;
use crate::models::Chat;

#[derive(Message)]
pub struct Connect {
    pub id: Uuid,
    pub addr: Recipient<Chat>
}

#[derive(Message)]
pub struct Disconnect {
    pub id: Uuid
}
