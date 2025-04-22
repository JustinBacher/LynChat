use actix::{Actor, StreamHandler};
use actix_web::{web, HttpRequest, HttpResponse, Error, get};
use actix_web_actors::ws;

pub struct ChatWs;

impl Actor for ChatWs {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            ctx.text(text);
        }
    }
}

#[get("/ws/chat")]
pub async fn ws_chat_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(ChatWs {}, &req, stream)
}