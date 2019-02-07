use actix_web::{AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Json, State, ws, error};
use actix_web::fs::NamedFile;
use futures::Future;
use uuid::Uuid;
use crate::models::WsSession;
use crate::handlers::db_messages::{User, Transcript};
use crate::AppState;

pub fn get_index(_req: &HttpRequest<AppState>) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("vue/test.html")?)
}

pub fn login((user, state): (Json<User>, State<AppState>))
    -> FutureResponse<HttpResponse> {
    state.db
        .send(user.into_inner())
        .from_err()
        .and_then(|response| match response {
            Ok(chats) => Ok(HttpResponse::Ok().json(chats)),
            Err(_err) => Ok(HttpResponse::InternalServerError().json("Internal Server Error"))
        }).responder()
}

pub fn send_chat((transcript, state): (Json<Transcript>, State<AppState>))
    -> FutureResponse<HttpResponse> {
    let ws = state.ws.clone();
    state.db
        .send(transcript.into_inner())
        .from_err()
        .and_then(move |response| match response {
            Ok(result) => {
                ws.do_send(result);
                Ok(HttpResponse::Ok().json(()))
                },
            Err(_err) => Ok(HttpResponse::InternalServerError().json("Internal Server Error"))
        }).responder()
}

pub fn get_ws(req: &HttpRequest<AppState>) -> Result<HttpResponse, error::Error> {
    ws::start(req, WsSession(Uuid::nil(), std::time::Instant::now()))
}
