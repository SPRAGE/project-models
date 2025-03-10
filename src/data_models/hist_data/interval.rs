
#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr, Clone)]
#[repr(i8)]
pub enum Interval {
    Day = 0,
    Minute = 1,
    ThreeMinute = 2,
    FiveMinute = 3,
    TenMinute = 4,
    FifteenMinute = 5,
    ThirtyMinute = 6,
    SixtyMinute = 7,
}

