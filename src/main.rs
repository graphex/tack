use std::net::UdpSocket;
use std::str;

use serde_json::Value;

use crate::line_protocol::*;
use crate::tempest_messages::*;

mod line_protocol;
mod tempest_messages;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    {
        // docker likes to take over this port, see `sudo lsof -i:50222`
        let socket = UdpSocket::bind("0.0.0.0:50222")?;

        loop {
            // Receives a single datagram message on the socket. If `buf` is too small to hold
            // the message, it will be cut off.
            let mut buf = [0; 2048];
            let (amt, src) = socket.recv_from(&mut buf)?;

            // Redeclare `buf` as slice of the received data
            let buf = &mut buf[..amt];
            let msg = str::from_utf8(&buf).unwrap();
            println!("{}: {}", src, msg);

            let v: Value = serde_json::from_str(&msg)?;
            let tempest_message = Some(serde_json::from_value::<TempestMessage>(v)?);

            match tempest_message {
                Some(tm) => println!("{}", tm.to_lpr()),
                _ => (),
            };
        }
    } // the socket is closed here
    // Ok(())
}
