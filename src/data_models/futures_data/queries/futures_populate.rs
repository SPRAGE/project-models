pub static POPULATE_FUTURE_STAGING:&str = r#"
INSERT INTO futures_staging
SELECT
    f.base_exchange,
    f.name,
    f.expiry,
    f.dte,
    CAST(f.future_type AS Enum8('atm' = 1, 'add_to_base' = 2, 'liquid' = 3)) AS future_type,
    f.underlying,
    f.base_expiry,
    COALESCE(existing.add_to_base, 0) AS add_to_base,
    f.strike
FROM (
    SELECT
        enriched.base_exchange,
        enriched.name,
        enriched.expiry,
        abs(toRelativeDayNum(enriched.expiry) - toRelativeDayNum(now())) AS dte,
        CASE
            WHEN has(enriched.instrument_types, 'Fut') THEN 'liquid'
            WHEN abs(toRelativeDayNum(enriched.expiry) - toRelativeDayNum(now())) > 30 THEN 'add_to_base'
            ELSE 'atm'
        END AS future_type,
        COALESCE(eq.instrument_token, 0) AS underlying,
        lagInFrame(enriched.expiry, 1) OVER (
            PARTITION BY enriched.base_exchange, enriched.name
            ORDER BY enriched.expiry ASC
            ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
        ) AS base_expiry,
        arraySort(arrayFilter(x -> x != 0 AND x IS NOT NULL, groupArray(DISTINCT toFloat64(enriched.strike)))) AS strike
    FROM instruments AS enriched
    LEFT JOIN (
        SELECT base_exchange, name, instrument_token
        FROM instruments
        WHERE instrument_type = 'Eq'
    ) AS eq
    ON enriched.base_exchange = eq.base_exchange AND enriched.name = eq.name
    WHERE enriched.instrument_type != 'Eq'
    GROUP BY enriched.base_exchange, enriched.name, enriched.expiry
) AS f
LEFT JOIN futures AS existing
ON f.base_exchange = existing.base_exchange
   AND f.name = existing.name
   AND f.expiry = existing.expiry
"#;
