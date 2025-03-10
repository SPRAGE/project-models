
#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(i8)]
pub enum BaseExchange {
    Bse = 0,
    Mcx = 1,
    Nse = 2,
    Nseix = 3,
    Global = 4,
}


