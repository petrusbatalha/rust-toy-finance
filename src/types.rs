use serde::{Serialize, Deserialize, Deserializer};
// StringRecord(["type", "client", "tx", "amount"])
// StringRecord(["deposit", "1", "1", "1.0"])
// StringRecord(["deposit", "2", "2", "2.0"])
// StringRecord(["deposit", "1", "3", "2.0"])
// StringRecord(["withdrawal", "1", "4", "1.5"])
// StringRecord(["withdrawal", "2", "5", "3.0"])

#[derive(Deserialize, Debug)]
pub struct Transactions {
    #[serde(rename = "type")]
    transaction_type: String,
    client: u16,
    tx: u32,
    amount: Option<f64>
}

#[derive(Deserialize, Debug)]
pub enum TransactionType<'a> {
    Withdraw(&'a [u8]),
    Deposit(&'a [u8]),
    Dispute(&'a [u8])
}

#[derive(Serialize, Deserialize)]
pub struct Output {
    client: u16,
    available: f32,
    held: f32,
    total: f32,
    locked: bool,
}
