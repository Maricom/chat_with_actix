use actix::Handler;
use crate::models::{WsActor, Chat};
use super::ws_messages::*;

impl Handler<Chat> for WsActor {
    type Result = ();

    fn handle(&mut self, msg: Chat, _: &mut Self::Context) {
        for ws in self.0.values() {
            ws.do_send(msg.clone()/* this is not best solution */);
        }
    }
}

impl Handler<Connect> for WsActor {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) {
        self.0.insert(msg.id, msg.addr);
    }
}

impl Handler<Disconnect> for WsActor {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) {
        self.0.remove(&msg.id);
    }
}
