use serde::{Deserialize, Serialize};

mod unit;

pub use unit::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Detail {
    Text(String),
    Integer(i64),
    Bytes(Box<[u8]>),
    Timestamp(Timestamp),
    Boolean(bool),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Metadata {
    details: Vec<Detail>,
}

impl Metadata {
    pub fn new() -> Self {
        Self {
            details: Vec::with_capacity(0),
        }
    }

    pub fn push(&mut self, value: Detail) {
        self.details.push(value)
    }

    pub fn pop(&mut self) -> Option<Detail> {
        self.details.pop()
    }

    #[inline(always)]
    pub fn empty() -> Self {
        Self::new()
    }

    pub fn details(&self) -> &[Detail] {
        &self.details
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Timestamp {
    secs: u64,
}

pub trait ToTimestamp {
    fn to_timestamp(&self) -> Timestamp;
}

impl ToTimestamp for u64 {
    fn to_timestamp(&self) -> Timestamp {
        Timestamp::from_secs(*self)
    }
}

#[test]
fn test_timestamp_for_u64() {
    let val = 33u64;
    let _timestamp = val.to_timestamp();
    assert!(true, "incomplete implementation of to_timestamp for u64")
}

impl<T: chrono::TimeZone> ToTimestamp for chrono::DateTime<T> {
    fn to_timestamp(&self) -> Timestamp {
        Timestamp {
            secs: self.timestamp() as _,
        }
    }
}

impl<F: ToTimestamp> From<F> for Timestamp {
    fn from(value: F) -> Self {
        value.to_timestamp()
    }
}

impl ToTimestamp for chrono::NaiveDateTime {
    fn to_timestamp(&self) -> Timestamp {
        Timestamp {
            secs: self.timestamp() as _,
        }
    }
}

pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Timestamp {
    pub fn year(&self) -> u16 {
        todo!()
    }
    pub fn month(&self) -> Month {
        todo!()
    }
    pub fn day(&self) -> u8 {
        todo!()
    }
    pub fn hour(&self) -> u8 {
        todo!()
    }
    pub fn minute(&self) -> u8 {
        todo!()
    }
    pub fn second(&self) -> u8 {
        todo!()
    }

    pub fn from_secs(secs: u64) -> Self {
        Self { secs }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BufID {
    value: [u8; 16],
}

impl BufID {
    pub fn random() -> Self {
        Self::new(crate::random_bytes())
    }

    pub fn new(value: [u8; 16]) -> Self {
        Self { value }
    }

    pub fn to_string(&self) -> String {
        hex::encode(&self.value)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Nonce {
    pub nonce: u64,
}

impl Nonce {
    pub fn new(nonce: u64) -> Self {
        Nonce { nonce }
    }
}

impl From<u64> for Nonce {
    fn from(value: u64) -> Self {
        Nonce::new(value)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Position {
    pub pos: u64,
}

impl Position {
    pub fn new(pos: u64) -> Self {
        Position { pos }
    }

    pub fn pos(&self) -> u64 {
        self.pos
    }
}

impl From<u64> for Position {
    fn from(value: u64) -> Self {
        Position::new(value)
    }
}
