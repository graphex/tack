#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use actix::{Actor, System};
use tokio::select;
use tokio::signal::unix::SignalKind;

use crate::line_protocol::*;
use crate::receiver_actor::*;
use crate::sender_actor::*;

mod line_protocol;
mod tempest_messages;
mod receiver_actor;
mod sender_actor;

#[actix::main]
async fn main() {
    let _sendr = SenderActor.start();
    let recvr = ReceiverActor.start();

    recvr.do_send(Listen);

    let ctl_c_watch = tokio::signal::ctrl_c();
    let mut sigterm_watch = tokio::signal::unix::signal(SignalKind::terminate()).unwrap();
    let mut sighup_watch = tokio::signal::unix::signal(SignalKind::hangup()).unwrap();
    //OK, you can't register for SIGKILLs
    // let mut sigkill_watch = tokio::signal::unix::signal(SignalKind::from_raw(9)).unwrap();

    select! {
        _ = ctl_c_watch => {
            println!("Ctrl-C received, shutting down");
            stop_program();
        },
        _ = sigterm_watch.recv() => {
            println!("SIGTERM received, shutting down");
            stop_program();
        }
        _ = sighup_watch.recv() => {
            println!("SIGHUP received, shutting down");
            stop_program();
        }
        // _ = sigkill_watch.recv() => {
        //     println!("SIGKILL received, shutting down");
        //     stop_program();
        // }
    }

    fn stop_program() {
        System::current().stop();
    }

}

