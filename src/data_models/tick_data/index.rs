use crate::data_models::instrument_data::Instrument;

use kiteticker_async::Tick;
use kiteticker_async::OHLC;
use redis::RedisWrite;
use redis::ToRedisArgs;

#[derive(Debug, Clone, Default)]
pub struct Index {
    pub name: String,
    pub instrument_token: String,
    pub ohlc: OHLC,
    pub last_price: f64,
    pub net_change: f64,
    pub pct_returns: f64,
}

impl Index {
    pub fn from_instrument(instrument: Instrument) -> Self {
        Index {
            name: instrument.name.clone().unwrap(),
            instrument_token: instrument.instrument_token,
            ohlc: OHLC::default(),
            last_price: 0.0,
            net_change: 0.0,
            pct_returns: 0.0,
        }
    }

    pub fn update(&mut self, tick: &Tick) {
        if let Some(ohlc) = &tick.ohlc {
            self.ohlc.open = ohlc.open;
            self.ohlc.high = ohlc.high;
            self.ohlc.low = ohlc.low;
            self.ohlc.close = ohlc.close;
        }
        if let Some(last_price) = tick.last_price {
            self.last_price = last_price;
        }
        self.net_change = tick.net_change.unwrap_or_default();
        self.pct_returns = pct_change(self.last_price, self.ohlc.close);
    }
}
fn pct_change(current: f64, previous: f64) -> f64 {
    ((current - previous) / previous) * 100.0
}

impl ToRedisArgs for Index {
    fn write_redis_args<W>(&self, out: &mut W)
    where

        W: ?Sized + RedisWrite,
    {
        let mut fields:Vec<(String,String)> = Vec::new();
        fields.push(("name".to_string(), self.name.clone()));
        fields.push(("instrument_token".to_string(), self.instrument_token.clone()));
        fields.push(("ohlc.open".to_string(), self.ohlc.open.to_string()));
        fields.push(("ohlc.high".to_string(), self.ohlc.high.to_string()));
        fields.push(("ohlc.low".to_string(), self.ohlc.low.to_string()));
        fields.push(("ohlc.close".to_string(), self.ohlc.close.to_string()));
        fields.push(("last_price".to_string(), self.last_price.to_string()));
        fields.push(("net_change".to_string(), self.net_change.to_string()));
        fields.push(("pct_returns".to_string(), self.pct_returns.to_string()));

        for (key, value) in fields {
            out.write_arg(key.as_bytes());
            out.write_arg(value.as_bytes());
        }

    }

}
