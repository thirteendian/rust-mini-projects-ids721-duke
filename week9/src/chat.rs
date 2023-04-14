use actix::prelude::*;
use actix_web_actors::ws;
use std::collections::HashMap;
use uuid::Uuid;

pub struct ChatServer {
    sessions: HashMap<Uuid, Addr<ChatSession>>,
}

impl Default for ChatServer {
    fn default() -> ChatServer {
        ChatServer {
            sessions: HashMap::new(),
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

pub struct Connect {
    pub addr: Addr<ChatSession>,
}

impl Message for Connect {
    type Result = Uuid;
}

pub struct Disconnect {
    pub id: Uuid,
}

impl Message for Disconnect {
    type Result = ();
}

pub struct SendMessage {
    pub id: Uuid,
    pub msg: String,
}

impl Message for SendMessage {
    type Result = ();
}

pub struct ChatSession {
    id: Uuid,
    server: Addr<ChatServer>,
}

impl ChatSession {
    pub fn new(server: Addr<ChatServer>) -> Self {
        ChatSession {
            id: Uuid::new_v4(),
            server,
        }
    }
}

impl Actor for ChatSession {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let server = self.server.clone();
                let id = self.id;
                async move {
                    server
                        .send(SendMessage {
                            id,
                            msg: text.clone(),
                        })
                        .await
                        .unwrap();
                }
                .into_actor(self)
                .wait(ctx);
            }
            Ok(ws::Message::Close(_)) => {
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl Handler<Connect> for ChatServer {
    type Result = Uuid;

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        let id = Uuid::new_v4();
        self.sessions.insert(id, msg.addr);
        id
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<SendMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: SendMessage, _ctx: &mut Self::Context) -> Self::Result {
        for (id, session) in &self.sessions {
            if *id != msg.id {
                let _ = session.do_send(msg.clone());
            }
        }
    }
}

impl Handler<SendMessage> for ChatSession {
    type Result = ();

    fn handle(&mut self, msg: SendMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.msg);
    }
}

impl Handler<Disconnect> for ChatSession {
    type Result = ();

    fn handle(&mut self, _: Disconnect, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
    }
}

impl Drop for ChatSession {
    fn drop(&mut self) {
        let _ = self.server.do_send(Disconnect { id: self.id });
    }
}
