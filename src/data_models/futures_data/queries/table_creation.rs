pub static CREATE_FUTURES_STAGING_TABLE:&str = r#"
CREATE TABLE IF NOT EXISTS futures_staging
(
    base_exchange String,
    name String,
    expiry Date,
    dte UInt16,
    future_type Enum8('atm' = 1, 'add_to_base' = 2, 'liquid' = 3),
    underlying UInt64,
    base_expiry Date,
    add_to_base UInt8,
    strike Array(Float64)
) ENGINE = MergeTree()
ORDER BY (base_exchange, name, expiry);
"#;

pub static CREATE_FUTURES_TABLE:&str = r#"
CREATE TABLE IF NOT EXISTS futures
(
    base_exchange String,
    name String,
    expiry Date,
    dte UInt16,
    future_type Enum8('atm' = 1, 'add_to_base' = 2, 'liquid' = 3),
    underlying UInt64,
    base_expiry Date,
    add_to_base UInt8,
    strike Array(Float64)
) ENGINE = MergeTree()
ORDER BY (base_exchange, name, expiry);
"#;
