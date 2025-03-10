

#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(i8)]
pub enum Exchange {
    Bcd = 0,
    Bfo = 1,
    Bse = 2,
    Cds = 3,
    Mcx = 4,
    Nco = 5,
    Nfo = 6,
    Nse = 7,
    Nseix = 8,
    Global = 9,
}


