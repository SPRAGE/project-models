pub static COPY_FUTURES:&str = r#"
INSERT INTO futures
SELECT
    *
FROM futures_staging;
"#;