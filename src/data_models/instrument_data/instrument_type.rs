
#[derive(Debug, serde_repr::Serialize_repr, serde_repr::Deserialize_repr)]
#[repr(i8)]
pub enum InstrumentType {
    Eq = 0,
    Fut = 1,
    Ce = 2,
    Pe = 3,
}


