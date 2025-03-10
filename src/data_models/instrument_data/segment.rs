
#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(i8)]
pub enum Segment {
    BcdFut = 0,
    BcdOpt = 1,
    BfoFut = 2,
    BfoOpt = 3,
    Bse = 4,
    CdsFut = 5,
    CdsOpt = 6,
    Indices = 7,
    McxFut = 8,
    McxOpt = 9,
    Nco = 10,
    NcoFut = 11,
    NcoOpt = 12,
    NfoFut = 13,
    NfoOpt = 14,
    Nse = 15,
}

