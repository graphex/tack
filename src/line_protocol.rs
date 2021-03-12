// use alloc::string::String;
// use alloc::vec::Vec;
use core::fmt;

pub struct LprVec(pub Vec<Lpr>);

impl fmt::Display for LprVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.iter().map(|l| l.to_string()).collect::<Vec<String>>().join("\n"))
    }
}

#[derive(Deserialize, Debug)]
#[serde()]
pub struct Lpr {
    pub measurement: String,
    pub tags: Vec<(String, String)>,
    pub fields: Vec<(String, FieldValue)>,
    pub timestamp: i64,
}

pub trait LprConvertable {
    fn to_lpr(&self) -> LprVec;
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
