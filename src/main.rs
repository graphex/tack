use std::net::UdpSocket;
use std::{str, fmt};
use serde_json::Value;
use smallvec::SmallVec;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    {
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

            let v: Value = serde_json::from_str(msg)?;
            let tempest_message = Some(serde_json::from_value::<TempestMessage>(v)?);
            tempest_message.map(|tm| println!("{:?}", tm));
        }
    } // the socket is closed here
    // Ok(())
}

/*#[derive(Serialize, Deserialize, Debug)]
#[serde()]
struct TempestObs<'a> {
    serial_number: String,
    r#type: String,
    hub_sn: String,
    firmware_revision: Option<i8>,
    ob: Value,
    obs: Value,
    evt: Value,
    timestamp: i64,
    //seconds
    observation: FieldSet<'a>,
    event: FieldSet<'a>,
}

impl fmt::Display for TempestObs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} serial_number={} {}", self.r#type, self.serial_number, "observation=1")
    }
}

pub type FieldSet<'a> = SmallVec<[(String, FieldValue<'a>); 4]>;

pub enum FieldValue<'a> {
    I64(i64),
    F64(f64),
    String(&'a str),
    Boolean(bool),
}*/


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TempestMessage {
    EvtPrecip(EvtPrecip),
    EvtStrike(EvtStrike),
    RapidWind(RapidWind),
    ObsAir(ObsAir),
    ObsSky(ObsSky),
    ObsSt(ObsSt),
    DeviceStatus(DeviceStatus),
    HubStatus(HubStatus),
}


#[derive(Serialize, Deserialize, Debug)]
pub struct EvtPrecip {
    serial_number: String,
    hub_sn: String,
    evt: serde_json::Value,
}

impl EvtPrecip {}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvtStrike {
    serial_number: String,
    hub_sn: String,
    evt: serde_json::Value,
}

impl EvtStrike {}

#[derive(Serialize, Deserialize, Debug)]
pub struct RapidWind {
    serial_number: String,
    hub_sn: String,
    ob: serde_json::Value,
    // timestamp: i64,
    // wind_speed: f32,
    // wind_direction: i16,
}

impl RapidWind {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObsAir {
    serial_number: String,
    hub_sn: String,
    obs: serde_json::Value,
    firmware_revision: i16,
}

impl ObsAir {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObsSky {
    serial_number: String,
    hub_sn: String,
    obs: serde_json::Value,
    firmware_revision: i16,
}

impl ObsSky {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ObsSt {
    serial_number: String,
    hub_sn: String,
    obs: serde_json::Value,
    firmware_revision: i16,
}

impl ObsSt {}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeviceStatus {
    serial_number: String,
    hub_sn: String,
    timestamp: i64,
    uptime: i64,
    voltage: f32,
    firmware_revision: i16,
    rssi: i8,
    hub_rssi: i8,
    sensor_status: i16,
    debug: i8,
}

impl DeviceStatus {}

#[derive(Serialize, Deserialize, Debug)]
pub struct HubStatus {
    serial_number: String,
    firmware_revision: String,
    uptime: u32,
    rssi: i8,
    timestamp: i64,
    reset_flags: String,
    seq: i32,
    fs: serde_json::Value,
    radio_stats: serde_json::Value,
    mqtt_stats: serde_json::Value,
}

impl HubStatus {}
