use chrono::{DateTime, Utc};
use crate::data_models::hist_data::Interval;

#[derive(Debug, clickhouse::Row, serde::Serialize, serde::Deserialize)]
pub struct HistData {
    pub tradingsymbol: String,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: u64,

    #[serde(with = "clickhouse::serde::chrono::datetime")]
    pub datetime: DateTime<Utc>, // field name fixed, type corrected

    pub interval: Interval,
}
