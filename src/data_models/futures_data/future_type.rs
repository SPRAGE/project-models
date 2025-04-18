
#[derive(Debug)]
#[derive(serde_repr::Serialize_repr)]
#[derive(serde_repr::Deserialize_repr)]
#[derive(Clone)]
#[derive(PartialEq)]
#[repr(i8)]
pub enum FutureType {
    Atm = 1,
    AddToBase = 2,
    Liquid = 3,
}