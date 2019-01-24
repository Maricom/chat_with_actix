extern crate actix;
#[macro_use] extern crate diesel;

mod schema;
mod models;
mod routes;
mod handlers;

use actix::prelude::*;
use actix_web::{server, App, http::Method};
use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;
use std::env;
use std::collections::HashMap;
use crate::models::{DbActor, WsActor};
use crate::routes::{get_index, get_chats, login, send_chat, get_ws};

pub struct AppState {
    pub db: Addr<DbActor>,
    pub ws: Addr<WsActor>
}

fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let sys = actix::System::new("chat");

    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let db_address: Addr<DbActor> = SyncArbiter::start(4, move || DbActor(pool.clone()));
    let ws_address: Addr<WsActor> = Arbiter::start(|_| WsActor(HashMap::new()));

    server::new(move || App::with_state(AppState { db: db_address.clone(), ws: ws_address.clone()})
            .resource("/", |r| r.method(Method::GET).f(get_index))
            .resource("/get_chat", |r| r.method(Method::GET).with(get_chats))
            .resource("/login", |r| r.method(Method::POST).with(login))
            .resource("/get_ws", |r| r.method(Method::GET).f(get_ws))
            .resource("/send", |r| r.method(Method::POST).with(send_chat))
        )
        .bind("localhost:5555")
        .unwrap()
        .start();

    let _ = sys.run();
}
