use serde::{Serialize};
use chrono::NaiveDateTime;
use serde_with::serde_as;

pub mod user;
pub mod collection;

#[derive(Debug, Serialize)]
#[serde(untagged)]
#[serde_as]
pub enum Value {
    Integer(Option<i64>),
    Text(Option<String>),
    #[serde_as(as = "Option<ts_milliseconds>")]
    DateTime(Option<NaiveDateTime>),
}
