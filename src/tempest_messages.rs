use crate::line_protocol::{FieldValue, Lpr, LprVec, LprConvertable};
use serde_tuple::*;

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

impl LprConvertable for TempestMessage {
    fn to_lpr(&self) -> LprVec {
        match self {
            TempestMessage::EvtPrecip(tm) => tm.to_lpr(),
            TempestMessage::EvtStrike(tm) => tm.to_lpr(),
            TempestMessage::RapidWind(tm) => tm.to_lpr(),
            TempestMessage::ObsAir(tm) => tm.to_lpr(),
            TempestMessage::ObsSky(tm) => tm.to_lpr(),
            TempestMessage::ObsSt(tm) => tm.to_lpr(),
            TempestMessage::DeviceStatus(tm) => tm.to_lpr(),
            TempestMessage::HubStatus(tm) => tm.to_lpr(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct EvtPrecip {
    serial_number: String,
    hub_sn: String,
    evt: EvtPrecipEvt,
}

#[derive(Deserialize_tuple, Debug)]
pub struct EvtPrecipEvt {
    timestamp: i64,
}

impl LprConvertable for EvtPrecip {
    fn to_lpr(&self) -> LprVec {
        LprVec(vec!(Lpr {
            measurement: String::from("EvtPrecip"),
            tags: vec!(
                (String::from("serial_number"), self.serial_number.clone()),
                (String::from("hub_sn"), self.hub_sn.clone()),
            ),
            fields: vec!(
                (String::from("started"), FieldValue::Boolean(true)),
            ),
            timestamp: self.evt.timestamp,
        }))
    }
}

#[derive(Deserialize, Debug)]
pub struct EvtStrike {
    serial_number: String,
    hub_sn: String,
    evt: EvtStrikeEvt,
}

#[derive(Deserialize_tuple, Debug)]
pub struct EvtStrikeEvt {
    timestamp: i64,
    distance: i16,
    energy: i32,
}

impl LprConvertable for EvtStrike {
    fn to_lpr(&self) -> LprVec {
        LprVec(vec!(Lpr {
            measurement: String::from("EvtStrike"),
            tags: vec!(
                (String::from("serial_number"), self.serial_number.clone()),
                (String::from("hub_sn"), self.hub_sn.clone()),
            ),
            fields: vec!(
                (String::from("distance"), FieldValue::I64(i64::from(self.evt.distance))),
                (String::from("energy"), FieldValue::I64(i64::from(self.evt.energy))),
            ),
            timestamp: self.evt.timestamp,
        }))
    }
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

impl LprConvertable for RapidWind {
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

impl LprConvertable for ObsAir {
    fn to_lpr(&self) -> LprVec {
        let lprs = self.obs.iter().map(|ob| {
            Lpr {
                measurement: String::from("ObsAir"),
                tags: vec!(
                    (String::from("serial_number"), self.serial_number.clone()),
                    (String::from("hub_sn"), self.hub_sn.clone()),
                    (String::from("firmware_revision"), self.firmware_revision.clone().to_string()),
                ),
                fields: vec!(
                    (String::from("station_pressure"), FieldValue::F64(f64::from(ob.station_pressure))),
                    (String::from("air_temperature"), FieldValue::F64(f64::from(ob.air_temperature))),
                    (String::from("relative_humidity"), FieldValue::F64(f64::from(ob.relative_humidity))),
                    (String::from("strike_count"), FieldValue::I64(i64::from(ob.strike_count))),
                    (String::from("strike_avg_distance"), FieldValue::I64(i64::from(ob.strike_avg_distance))),
                    (String::from("battery"), FieldValue::F64(f64::from(ob.battery))),
                    (String::from("report_interval"), FieldValue::I64(i64::from(ob.report_interval))),
                ),
                timestamp: ob.timestamp,
            }
        });
        LprVec(lprs.collect())
    }
}

#[derive(Deserialize, Debug)]
pub struct ObsSky {
    serial_number: String,
    hub_sn: String,
    obs: Vec<ObsSkyOb>,
    firmware_revision: i16,
}

#[allow(dead_code)]
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

impl LprConvertable for ObsSky {
    fn to_lpr(&self) -> LprVec {
        let lprs = self.obs.iter().map(|ob| {
            let mut cur_lpr = Lpr {
                measurement: String::from("ObsSky"),
                tags: vec!(
                    (String::from("serial_number"), self.serial_number.clone()),
                    (String::from("hub_sn"), self.hub_sn.clone()),
                    (String::from("firmware_revision"), self.firmware_revision.clone().to_string()),
                ),
                fields: vec!(
                    (String::from("illuminance"), FieldValue::I64(i64::from(ob.illuminance))),
                    (String::from("uv"), FieldValue::F64(f64::from(ob.uv))),
                    (String::from("rain_acc"), FieldValue::F64(f64::from(ob.rain_acc))),
                    (String::from("wind_lull"), FieldValue::F64(f64::from(ob.wind_lull))),
                    (String::from("wind_avg"), FieldValue::F64(f64::from(ob.wind_avg))),
                    (String::from("wind_gust"), FieldValue::F64(f64::from(ob.wind_gust))),
                    (String::from("wind_direction"), FieldValue::I64(i64::from(ob.wind_direction))),
                    (String::from("battery"), FieldValue::F64(f64::from(ob.battery))),
                    (String::from("report_interval"), FieldValue::I64(i64::from(ob.report_interval))),
                    (String::from("solar_radiation"), FieldValue::I64(i64::from(ob.solar_radiation))),
                ),
                timestamp: ob.timestamp,
            };
            ob.day_rain_acc.map(|dra| cur_lpr.fields.push(
                (String::from("day_rain_acc"), FieldValue::I64(i64::from(dra))),
            ));
            cur_lpr
        });
        LprVec(lprs.collect())
    }
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
    wind_lull: Option<f32>,
    wind_avg: Option<f32>,
    wind_gust: Option<f32>,
    wind_direction: Option<i16>,
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

impl LprConvertable for ObsSt {
    fn to_lpr(&self) -> LprVec {
        let lprs = self.obs.iter().map(|ob| {
            let mut cur_lpr = Lpr {
                measurement: String::from("ObsSt"),
                tags: vec!(
                    (String::from("serial_number"), self.serial_number.clone()),
                    (String::from("hub_sn"), self.hub_sn.clone()),
                    (String::from("firmware_revision"), self.firmware_revision.clone().to_string()),
                ),
                fields: vec!(
                    (String::from("wind_sample_interval"), FieldValue::I64(i64::from(ob.wind_sample_interval))),
                    (String::from("air_temperature"), FieldValue::F64(f64::from(ob.air_temperature))),
                    (String::from("relative_humidity"), FieldValue::F64(f64::from(ob.relative_humidity))),
                    (String::from("illuminance"), FieldValue::I64(i64::from(ob.illuminance))),
                    (String::from("uv"), FieldValue::F64(f64::from(ob.uv))),
                    (String::from("solar_radiation"), FieldValue::I64(i64::from(ob.solar_radiation))),
                    (String::from("precip_acc"), FieldValue::F64(f64::from(ob.precip_acc))),
                    (String::from("precip_type"), FieldValue::I64(i64::from(ob.precip_type))),
                    (String::from("strike_avg_distance"), FieldValue::I64(i64::from(ob.strike_avg_distance))),
                    (String::from("strike_count"), FieldValue::I64(i64::from(ob.strike_count))),
                    (String::from("battery"), FieldValue::F64(f64::from(ob.battery))),
                    (String::from("report_interval"), FieldValue::I64(i64::from(ob.report_interval))),
                ),
                timestamp: ob.timestamp,
            };
            let mut opts: Vec<(String, FieldValue)> = vec![];
            ob.wind_lull.map(|v| opts.push(
                (String::from("wind_lull"), FieldValue::F64(f64::from(v))),
            ));
            ob.wind_avg.map(|v| opts.push(
                (String::from("wind_avg"), FieldValue::F64(f64::from(v))),
            ));
            ob.wind_gust.map(|v| opts.push(
                (String::from("wind_gust"), FieldValue::F64(f64::from(v))),
            ));
            ob.wind_direction.map(|v| opts.push(
                (String::from("wind_direction"), FieldValue::F64(f64::from(v))),
            ));
            cur_lpr.fields = opts.into_iter().chain(cur_lpr.fields.into_iter()).collect();
            ob.station_pressure.map(|v| cur_lpr.fields.push(
                (String::from("station_pressure"), FieldValue::F64(f64::from(v))),
            ));

            cur_lpr
        });
        LprVec(lprs.collect())
    }
}

#[allow(dead_code)]
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

impl LprConvertable for DeviceStatus {
    fn to_lpr(&self) -> LprVec {
        LprVec(vec!(Lpr {
            measurement: String::from("DeviceStatus"),
            tags: vec!(
                (String::from("serial_number"), self.serial_number.clone()),
                (String::from("hub_sn"), self.hub_sn.clone()),
                (String::from("firmware_revision"), self.firmware_revision.clone().to_string()),
            ),
            fields: vec!(
                (String::from("uptime"), FieldValue::I64(i64::from(self.uptime))),
                (String::from("voltage"), FieldValue::F64(f64::from(self.voltage))),
                (String::from("rssi"), FieldValue::I64(i64::from(self.rssi))),
                (String::from("sensor_status"), FieldValue::I64(i64::from(self.sensor_status))),
                (String::from("debug"), FieldValue::I64(i64::from(self.debug))),
            ),
            timestamp: self.timestamp,
        }))
    }
}

#[allow(dead_code)]
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

impl LprConvertable for HubStatus {
    fn to_lpr(&self) -> LprVec {
        LprVec(vec!(Lpr {
            measurement: String::from("HubStatus"),
            tags: vec!(
                //Note: since this is the hub S/N, report it as hub_sn for consistency with other data
                (String::from("hub_sn"), self.serial_number.clone()),
                (String::from("firmware_revision"), self.firmware_revision.clone().to_string()),
            ),
            fields: vec!(
                (String::from("uptime"), FieldValue::I64(i64::from(self.uptime))),
                (String::from("rssi"), FieldValue::I64(i64::from(self.rssi))),
                // (String::from("reset_flags"), FieldValue::String(self.reset_flags.clone().to_string())),
                (String::from("seq"), FieldValue::I64(i64::from(self.seq))),
            ),
            timestamp: self.timestamp,
        }))
    }
}
