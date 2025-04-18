use crate::data_models::instrument_data::BaseExchange;
use crate::data_models::futures_data::FutureType;

#[derive(Debug, clickhouse::Row)]
#[derive(serde::Serialize)]
#[derive(serde::Deserialize)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct FuturesData {
    pub base_exchange: BaseExchange,
    pub name: String,
    pub expiry: u16,
    pub dte: u16,
    pub future_type: FutureType,
    pub underlying: u64,
    pub base_expiry: u16,
    pub add_to_base: u8,
    pub strike: Vec<f64>,
}