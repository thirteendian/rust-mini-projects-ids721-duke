use actix::prelude::*;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use futures::StreamExt;
use tungstenite::Message;

mod chat;

use crate::chat::{ChatServer, Connect, Disconnect, SendMessage};

async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        ChatSession::new(srv.get_ref().clone()),
        &req,
        stream,
    )
}

fn main() -> std::io::Result<()> {
    let sys = System::new();

    let server_addr = "127.0.0.1:8080";
    let server = HttpServer::new(move || {
        App::new()
            .data(ChatServer::default().start())
            .service(web::resource("/ws/").to(chat_route))
    })
    .bind(server_addr)?;

    println!("Starting server at {}", server_addr);
    server.run()?;
    sys.run()
}
