pub static CREATE_INSTRUMENTS_TABLE:&str = r#"
CREATE TABLE IF NOT EXISTS data.instruments
(
    instrument_token String,
    exchange_token String,
    tradingsymbol String,
    name Nullable(String),
    last_price Float64,
    expiry Nullable(Date),
    strike Float64,
    tick_size Float64,
    lot_size UInt32,
    exchange Enum8(
        'Bcd' = 0, 'Bfo' = 1, 'Bse' = 2, 'Cds' = 3,
        'Mcx' = 4, 'Nco' = 5, 'Nfo' = 6, 'Nse' = 7,
        'Nseix' = 8, 'Global' = 9
    ),
    segment Enum8(
        'BcdFut' = 0, 'BcdOpt' = 1, 'BfoFut' = 2, 'BfoOpt' = 3,
        'Bse' = 4, 'CdsFut' = 5, 'CdsOpt' = 6, 'Indices' = 7,
        'McxFut' = 8, 'McxOpt' = 9, 'Nco' = 10, 'NcoFut' = 11,
        'NcoOpt' = 12, 'NfoFut' = 13, 'NfoOpt' = 14, 'Nse' = 15
    ),
    instrument_type Enum8(
        'Eq' = 0, 'Fut' = 1, 'Ce' = 2, 'Pe' = 3
    ),
    base_exchange Enum8(
        'Bse' = 0, 'Mcx' = 1, 'Nse' = 2, 'Nseix' = 3, 'Global' = 4
    )
)
ENGINE = MergeTree()
ORDER BY (instrument_token)"#;
