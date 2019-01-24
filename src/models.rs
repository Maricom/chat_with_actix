use crate::actix::prelude::*;
use crate::actix::{Actor, SyncContext, Running, StreamHandler};
use actix_web::ws;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use serde_derive::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use uuid::Uuid;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use crate::AppState;
use crate::schema::chats;
use crate::handlers::ws_messages::{Connect, Disconnect};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Message)]
#[table_name = "chats"]
pub struct Chat {
    pub id: Uuid,
    pub user_name: String,
    pub body: String,
    pub ts: NaiveDateTime
}

pub struct DbActor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

pub struct WsActor(pub HashMap<Uuid, Recipient<Chat>>);

impl Actor for WsActor {
    type Context = Context<Self>;
}

pub struct WsSession(pub Uuid, pub Instant);

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self, AppState>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let id = Uuid::new_v4();
        self.0 = id;
        
        let addr = ctx.address().recipient();
        ctx.state()
            .ws
            .do_send(Connect {
                id,
                addr
            });
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> Running {
        ctx.state().ws.do_send(Disconnect { id: self.0 });
        Running::Stop
    }
}

impl WsSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self, AppState>) {
        ctx.run_interval(Duration::from_secs(5), |act, ctx| {
            if Instant::now().duration_since(act.1) > Duration::from_secs(10) {
                println!("Websocket Client heartbeat failed, disconnecting!");

                ctx.state()
                    .ws
                    .do_send(Disconnect { id: act.0 });
                
                ctx.stop();

                return;
            }

            ctx.ping("");
        });
    }
}

impl Handler<Chat> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: Chat, ctx: &mut Self::Context) {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}

impl StreamHandler<ws::Message, ws::ProtocolError> for WsSession {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        // sprintln!("WEBSOCKET MESSAGE: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => {
                self.1 = Instant::now();
                ctx.pong(&msg);
            },
            ws::Message::Pong(_) => {
                self.1 = Instant::now();
            },
            ws::Message::Text(_) => {},
            ws::Message::Binary(_bin) => println!("Unexpected binary"),
            ws::Message::Close(_) => {
                ctx.stop();
            },
        }
    }
}
