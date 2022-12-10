use std::net::UdpSocket;
use std::str;
use actix::{Actor, Addr, Context, Registry, System};
use serde_json::Value;
use std::error::Error;
use actix::prelude::*;
use tokio::io::WriteHalf;
use tokio::net::TcpStream;

use crate::line_protocol::*;
use crate::tempest_messages::*;
use crate::receiver_actor::*;
use crate::sender_actor::*;

mod line_protocol;
mod tempest_messages;
mod receiver_actor;
mod sender_actor;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[actix::main]
async fn main() {
    let sendr = SenderActor.start();
    let recvr = ReceiverActor.start();
    recvr.do_send(Listen);
    tokio::signal::ctrl_c().await.unwrap();
    println!("Ctrl-C received, shutting down");
    System::current().stop();
}

