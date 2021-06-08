use std::error::Error;
use std::str;
use bytes::Bytes;
use bytes::BytesMut;
use actix::io::SinkWrite;
use actix::{Actor, Addr, AsyncContext, Context, System};
use actix::prelude::*;
use serde_json::Value;
use tokio::net::UdpSocket;
use tokio_util::udp::UdpFramed;
use tokio_util::codec::BytesCodec;

use crate::tempest_messages::*;
use crate::line_protocol::LprConvertable;
use futures::StreamExt;
use std::net::SocketAddr;

pub struct ReceiverActor;

impl ReceiverActor {
    pub fn new() -> ReceiverActor {
        ReceiverActor
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Listen;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ReceiveDatagram(BytesMut, SocketAddr);

impl Actor for ReceiverActor {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Hello, world!");
    }
}

impl StreamHandler<ReceiveDatagram> for ReceiverActor {
    fn handle(&mut self, msg: ReceiveDatagram, ctx: &mut Self::Context) {
        // println!("Received: ({:?}, {:?})", msg.0, msg.1);
        let msg = str::from_utf8(msg.0.as_ref()).unwrap();
        // println!("Raw: {}", msg.to_string());
        let v: Value = serde_json::from_str(&msg).unwrap();
        let tempest_message = Some(serde_json::from_value::<TempestMessage>(v).unwrap());
        match tempest_message {
            Some(tm) => println!("{}", tm.to_lpr()),
            _ => (),
        };
    }
}

impl Handler<Listen> for ReceiverActor {
    type Result = ();
    fn handle(&mut self, msg: Listen, ctx: &mut Context<Self>) {
        /*
        docker likes to take over this port, see `sudo lsof -i:50222`
        let addr: SocketAddr = "0.0.0.0:50222".parse().unwrap();
        let socket = UdpSocket::bind(&addr).await.unwrap();
        HACK: Unable to figure out how to run the async tokio bind above so setting up a std socket
        and converting it to a tokio socket instead
        */
        let std_socket = std::net::UdpSocket::bind("0.0.0.0:50222").unwrap();
        std_socket.set_nonblocking(true);
        let socket = UdpSocket::from_std(std_socket).unwrap();
        let (_, stream) = UdpFramed::new(socket, BytesCodec::new()).split();
        ctx.add_stream(stream.filter_map(
            |item| async {
                item.map(|(data, sender)| ReceiveDatagram(data, sender)).ok()
            },
        ));
    }
}
