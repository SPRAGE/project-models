pub static CREATE_HIST_DATA_TABLE:&str = r#"
CREATE TABLE IF NOT EXISTS historical_data
(
    tradingsymbol String,
    open Float64,
    high Float64,
    low Float64,
    close Float64,
    volume UInt64,
    datetime DateTime('Asia/Kolkata'),
    interval Enum8(
        'Day' = 0,
        'Minute' = 1,
        'ThreeMinute' = 2,
        'FiveMinute' = 3,
        'TenMinute' = 4,
        'FifteenMinute' = 5,
        'ThirtyMinute' = 6,
        'SixtyMinute' = 7
    )
)
ENGINE = ReplacingMergeTree
PARTITION BY (tradingsymbol, interval)
ORDER BY (tradingsymbol, interval, datetime)
"#;
