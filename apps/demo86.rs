use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Msg {
    pub room: String,
    pub username: String,
    pub timestamp: u64,
}

impl TryFrom<&str> for Msg {
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }
}

impl TryFrom<&Msg> for String {
    type Error = serde_json::Error;

    fn try_from(value: &Msg) -> Result<Self, Self::Error> {
        serde_json::to_string(value)
    }
}

fn main() -> Result<()> {
    let msg = &Msg {
        room: "1".into(),
        username: "2".into(),
        timestamp: 0,
    };

    let x: String = msg.try_into()?;
    dbg!(&x);

    let x1: Msg = x.as_str().try_into()?;
    dbg!(x1);

    Ok(())
}
