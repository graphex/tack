use std::net::UdpSocket;
use std::str;
use serde_json::Value;
use serde_tuple::*;
// use serde::Serialize;
// use std::collections::HashMap;
use core::fmt;

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

            let v: Value = serde_json::from_str(&msg)?;
            let tempest_message = Some(serde_json::from_value::<TempestMessage>(v)?);

            match tempest_message {
                Some(TempestMessage::RapidWind(rw)) => println!("{}", rw.to_lpr()),
                Some(TempestMessage::ObsSt(obsSt)) => println!("{}", obsSt.to_lpr()),
                Some(tm) => println!("{:?}", tm),
                _ => (),
            };
        }
    } // the socket is closed here
    // Ok(())
}

struct LprVec(Vec<Lpr>);

impl fmt::Display for LprVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.iter().map(|l| l.to_string()).collect::<Vec<String>>().join("\n"))
    }
}

#[derive(Deserialize, Debug)]
#[serde()]
struct Lpr {
    measurement: String,
    tags: Vec<(String, String)>,
    fields: Vec<(String, FieldValue)>,
    timestamp: i64,
}

impl fmt::Display for Lpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tag_str = self.tags.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join(",");
        let field_str = self.fields.iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{},{} {} {}", self.measurement, tag_str, field_str, self.timestamp)
    }
}

#[derive(Deserialize, Debug)]
pub enum FieldValue {
    I64(i64),
    F64(f64),
    String(String),
    Boolean(bool),
}

impl fmt::Display for FieldValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldValue::I64(v) => write!(f, "{}", v),
            FieldValue::F64(v) => write!(f, "{:.2}", v),
            FieldValue::String(v) => write!(f, "{}", v),
            FieldValue::Boolean(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Deserialize, Debug)]
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


#[derive(Deserialize, Debug)]
pub struct EvtPrecip {
    serial_number: String,
    hub_sn: String,
    evt: EvtPrecipEvt,
}

impl EvtPrecip {}

#[derive(Deserialize_tuple, Debug)]
pub struct EvtPrecipEvt {
    timestamp: i64,
}

#[derive(Deserialize, Debug)]
pub struct EvtStrike {
    serial_number: String,
    hub_sn: String,
    evt: EvtStrikeEvt,
}

impl EvtStrike {}

#[derive(Deserialize_tuple, Debug)]
pub struct EvtStrikeEvt {
    timestamp: i64,
    distance: i16,
    wind_direction: i32,
}

#[derive(Deserialize, Debug)]
pub struct RapidWind {
    serial_number: String,
    hub_sn: String,
    ob: RapidWindOb,
}

#[derive(Serialize_tuple, Deserialize_tuple, Debug)]
pub struct RapidWindOb {
    timestamp: i64,
    wind_speed: f32,
    wind_direction: i16,
}

impl RapidWind {
    fn to_lpr(&self) -> LprVec {
        LprVec(vec!(Lpr {
            measurement: String::from("RapidWind"),
            tags: vec!(
                (String::from("serial_number"), self.serial_number.clone()),
                (String::from("hub_sn"), self.hub_sn.clone()),
            ),
            fields: vec!(
                (String::from("wind_speed"), FieldValue::F64(f64::from(self.ob.wind_speed))),
                (String::from("wind_direction"), FieldValue::I64(i64::from(self.ob.wind_direction))),
            ),
            timestamp: self.ob.timestamp,
        }))
    }
}


#[derive(Deserialize, Debug)]
pub struct ObsAir {
    serial_number: String,
    hub_sn: String,
    obs: Vec<ObsAirOb>,
    firmware_revision: i16,
}

impl ObsAir {}

#[derive(Deserialize_tuple, Debug)]
pub struct ObsAirOb {
    timestamp: i64,
    station_pressure: f32,
    air_temperature: f32,
    relative_humidity: f32,
    strike_count: u8,
    strike_avg_distance: u8,
    battery: f32,
    report_interval: u8,
}

#[derive(Deserialize, Debug)]
pub struct ObsSky {
    serial_number: String,
    hub_sn: String,
    obs: Vec<ObsSkyOb>,
    firmware_revision: i16,
}

impl ObsSky {}

#[derive(Deserialize_tuple, Debug)]
pub struct ObsSkyOb {
    timestamp: i64,
    illuminance: u32,
    uv: f32,
    rain_acc: f32,
    wind_lull: f32,
    wind_avg: f32,
    wind_gust: f32,
    wind_direction: i16,
    battery: f32,
    report_interval: u8,
    solar_radiation: u16,
    day_rain_acc: Option<u8>,
    precip_type: u8,
    wind_sample_interval: u16,
}

#[derive(Deserialize, Debug)]
pub struct ObsSt {
    serial_number: String,
    hub_sn: String,
    obs: Vec<ObsStOb>,
    firmware_revision: i16,
}

#[derive(Deserialize_tuple, Debug)]
pub struct ObsStOb {
    timestamp: i64,
    wind_lull: f32,
    wind_avg: f32,
    wind_gust: f32,
    wind_direction: i16,
    wind_sample_interval: u16,
    station_pressure: Option<f32>,
    air_temperature: f32,
    relative_humidity: f32,
    illuminance: u32,
    uv: f32,
    solar_radiation: u16,
    precip_acc: f32,
    precip_type: u8,
    strike_avg_distance: u8,
    strike_count: u8,
    battery: f32,
    report_interval: u8,
}

impl ObsSt {
    fn to_lpr(&self) -> LprVec {
        let lprs = self.obs.iter().map(|ob|
            Lpr {
                measurement: String::from("ObsSt"),
                tags: vec!(
                    (String::from("serial_number"), self.serial_number.clone()),
                    (String::from("hub_sn"), self.hub_sn.clone()),
                    (String::from("firmware_revision"), self.firmware_revision.clone().to_string()),
                ),
                fields: vec!(
                    (String::from("wind_lull"), FieldValue::F64(f64::from(ob.wind_lull))),
                    (String::from("wind_avg"), FieldValue::F64(f64::from(ob.wind_avg))),
                    (String::from("wind_gust"), FieldValue::F64(f64::from(ob.wind_gust))),
                    (String::from("wind_direction"), FieldValue::I64(i64::from(ob.wind_direction))),
                    (String::from("wind_sample_interval"), FieldValue::I64(i64::from(ob.wind_sample_interval))),
                ),
                timestamp: ob.timestamp,
            }
        );
        LprVec(lprs.collect())
    }
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
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
