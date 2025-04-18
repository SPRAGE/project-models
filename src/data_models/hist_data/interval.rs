
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

impl Interval {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "Day" => Some(Interval::Day),
            "Minute" => Some(Interval::Minute),
            "ThreeMinute" => Some(Interval::ThreeMinute),
            "FiveMinute" => Some(Interval::FiveMinute),
            "TenMinute" => Some(Interval::TenMinute),
            "FifteenMinute" => Some(Interval::FifteenMinute),
            "ThirtyMinute" => Some(Interval::ThirtyMinute),
            "SixtyMinute" => Some(Interval::SixtyMinute),
            _ => None,
        }
    }
    pub fn from_integer(i: i8) -> Option<Self> {
        match i {
            0 => Some(Interval::Day),
            1 => Some(Interval::Minute),
            2 => Some(Interval::ThreeMinute),
            3 => Some(Interval::FiveMinute),
            4 => Some(Interval::TenMinute),
            5 => Some(Interval::FifteenMinute),
            6 => Some(Interval::ThirtyMinute),
            7 => Some(Interval::SixtyMinute),
            _ => None,
        }
    }
}
