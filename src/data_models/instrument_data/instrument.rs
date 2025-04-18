use crate::data_models::instrument_data::Exchange;
use crate::data_models::instrument_data::InstrumentType;
use crate::data_models::instrument_data::Segment;
use crate::data_models::instrument_data::BaseExchange;
use chrono::NaiveDate;
use serde_json::Value;
use std::error::Error;

#[derive(Debug, clickhouse::Row, serde::Serialize, serde::Deserialize)]
pub struct Instrument {
    pub instrument_token: String,
    pub exchange_token: String,
    pub tradingsymbol: String,
    pub name: Option<String>,
    pub last_price: f64,
    #[serde(with = "clickhouse::serde::chrono::date::option")]
    pub expiry: Option<NaiveDate>,
    pub strike: f64,
    pub tick_size: f64,
    pub lot_size: u32,
    pub exchange: Exchange,
    pub segment: Segment,
    pub instrument_type: InstrumentType,
    pub base_exchange: BaseExchange,
}

impl Instrument {
    /// Creates an `Instrument` instance from a JSON `Value`.
    pub fn from_json(item: &Value) -> Result<Self, Box<dyn Error>> {
        let format = "%Y-%m-%d";

        Ok(Instrument {
            exchange: match item.get("exchange").and_then(|v| v.as_str()) {
                Some("NSE") => Exchange::Nse,
                Some("BSE") => Exchange::Bse,
                Some("MCX") => Exchange::Mcx,
                Some("NSEIX") => Exchange::Nseix,
                Some("BCD") => Exchange::Bcd,
                Some("BFO") => Exchange::Bfo,
                Some("CDS") => Exchange::Cds,
                Some("NCO") => Exchange::Nco,
                Some("NFO") => Exchange::Nfo,
                Some("GLOBAL") => Exchange::Global,
                _ => return Err("Invalid or missing `exchange` value".into()),
            },

            exchange_token: item
                .get("exchange_token")
                .and_then(|v| v.as_str())
                .ok_or("Missing `exchange_token`")?
                .to_string(),

            expiry: item.get("expiry").and_then(|v| v.as_str()).and_then(|s| {
                if !s.is_empty() {
                    NaiveDate::parse_from_str(s, &format).ok()
                } else {
                    None
                }
            }),

            instrument_token: item
                .get("instrument_token")
                .and_then(|v| v.as_str())
                .ok_or("Missing `instrument_token`")?
                .to_string(),

            instrument_type: match item.get("instrument_type").and_then(|v| v.as_str()) {
                Some("EQ") => InstrumentType::Eq,
                Some("FUT") => InstrumentType::Fut,
                Some("CE") => InstrumentType::Ce,
                Some("PE") => InstrumentType::Pe,
                _ => return Err("Invalid or missing `instrument_type` value".into()),
            },

            last_price: item
                .get("last_price")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0),
            lot_size: item
                .get("lot_size")
                .and_then(|v| {
                    if let Some(u) = v.as_u64() {
                        Some(u as u32)
                    } else if let Some(s) = v.as_str() {
                        s.parse::<u32>().ok()
                    } else {
                        None
                    }
                })
                .ok_or("Missing or invalid `lot_size`")?,

            name: item.get("name").and_then(|v| v.as_str()).map(|s| match s {
                "NIFTY 50" => "NIFTY".to_string(),
                "NIFTY BANK" => "BANKNIFTY".to_string(),
                "NIFTY MIDCAP SELECT (MIDCPNIFTY)" => "MIDCPNIFTY".to_string(),
                "NIFTY NEXT 50" => "NIFTYNXT50".to_string(),
                "NIFTY FIN SERVICE" => "FINNIFTY".to_string(),
                "BSE INDEX BANKEX" => "BANKEX".to_string(),
                _ => s.to_string(),
            }),

            segment: match item.get("segment").and_then(|v| v.as_str()) {
                Some("BCD-FUT") => Segment::BcdFut,
                Some("BCD-OPT") => Segment::BcdOpt,
                Some("BFO-FUT") => Segment::BfoFut,
                Some("BFO-OPT") => Segment::BfoOpt,
                Some("BSE") => Segment::Bse,
                Some("CDS-FUT") => Segment::CdsFut,
                Some("CDS-OPT") => Segment::CdsOpt,
                Some("INDICES") => Segment::Indices,
                Some("MCX-FUT") => Segment::McxFut,
                Some("MCX-OPT") => Segment::McxOpt,
                Some("NCO") => Segment::Nco,
                Some("NCO-FUT") => Segment::NcoFut,
                Some("NCO-OPT") => Segment::NcoOpt,
                Some("NFO-FUT") => Segment::NfoFut,
                Some("NFO-OPT") => Segment::NfoOpt,
                Some("NSE") => Segment::Nse,
                _ => return Err("Invalid or missing `segment` value".into()),
            },

            strike: item.get("strike").and_then(|v| {
                if let Some(f) = v.as_f64() {
                    Some(f)
                } else if let Some(s) = v.as_str() {
                    s.parse::<f64>().ok()
                } else {
                    None
                }
            }).unwrap_or(0.0),
            tick_size: item
                .get("tick_size")
                .and_then(|v| v.as_f64())
                .unwrap_or(0.01),

            tradingsymbol: item
                .get("tradingsymbol")
                .and_then(|v| v.as_str())
                .ok_or("Missing `tradingsymbol`")?
                .to_string(),

            base_exchange: match item.get("exchange").and_then(|v| v.as_str()) {
                Some("NSE") => BaseExchange::Nse,
                Some("BSE") => BaseExchange::Bse,
                Some("MCX") => BaseExchange::Mcx,
                Some("NSEIX") => BaseExchange::Nseix,
                Some("BCD") => BaseExchange::Bse,
                Some("BFO") => BaseExchange::Bse,
                Some("CDS") => BaseExchange::Nse,
                Some("NCO") => BaseExchange::Nse,
                Some("NFO") => BaseExchange::Nse,
                Some("GLOBAL") => BaseExchange::Global,
                _ => return Err("Invalid or missing `exchange` value".into()),
            },
        })
    }
}
