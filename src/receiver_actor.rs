use std::error::Error;
use std::net::UdpSocket;
use std::str;

use actix::{Actor, Addr, Context, System};
use actix::prelude::*;
use serde_json::Value;

use crate::tempest_messages::*;
use crate::line_protocol::LprConvertable;

pub struct ReceiverActor;

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct Listen;

impl Actor for ReceiverActor {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("Hello, world!");
    }
}

impl Handler<Listen> for ReceiverActor {
    type Result = ();
    fn handle(&mut self, msg: Listen, ctx: &mut Context<Self>) {
        // docker likes to take over this port, see `sudo lsof -i:50222`
        let socketResult = UdpSocket::bind("0.0.0.0:50222");
        let socket = socketResult.unwrap();
        loop {
            // Receives a single datagram message on the socket. If `buf` is too small to hold
            // the message, it will be cut off.
            let mut buf = [0; 2048];
            let (amt, src) = socket.recv_from(&mut buf).unwrap();

            // Redeclare `buf` as slice of the received data
            let buf = &mut buf[..amt];
            let msg = str::from_utf8(&buf).unwrap();
            println!("{}: {}", src, msg);

            let v: Value = serde_json::from_str(&msg).unwrap();
            let tempest_message = Some(serde_json::from_value::<TempestMessage>(v).unwrap());

            match tempest_message {
                Some(tm) => println!("{}", tm.to_lpr()),
                _ => (),
            };
        };
    }
}
