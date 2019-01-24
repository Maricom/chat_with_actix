use actix::Handler;
use actix_web::error;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::Local;
use super::db_messages::*;
use crate::models::{DbActor, Chat};
use crate::schema;

impl Handler<User> for DbActor {
    type Result = Result<Vec<Chat>, error::Error>;

    fn handle(&mut self, _msg: User, _: &mut Self::Context) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();

        let chats = schema::chats::table
            .order(schema::chats::ts.desc())
            .limit(20)
            .load::<Chat>(conn)
            .unwrap();

        Ok(chats)
    }
}

impl Handler<Transcript> for DbActor {
    type Result = Result<Chat, error::Error>;

    fn handle(&mut self, msg: Transcript, _: &mut Self::Context) -> Self::Result {
        let conn: &PgConnection = &self.0.get().unwrap();

        let new_chat = Chat {
            id: Uuid::new_v4(),
            user_name: msg.user_name,
            body: msg.body,
            ts: Local::now().naive_local()
        };

        let result = diesel::insert_into(schema::chats::table)
            .values(&new_chat)
            .get_result(conn)
            .unwrap();
        
        Ok(result)
    }
}
