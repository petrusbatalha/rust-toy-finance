use serde::{Serialize, Deserialize, Deserializer};
use std::borrow::Cow;
use serde::de::{IntoDeserializer, Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transactions<'a> {
    #[serde(rename = "type")]
    #[serde(borrow)]
    transaction_type: TransactionType<'a>,
    client: u16,
    tx: u32,
    amount: Option<f64>
}

#[derive(Debug, Serialize)]
pub enum TransactionType<'a> {
    #[serde(rename = "dispute")]
    #[serde(borrow)]
    Dispute(Cow<'a, str>,),
    #[serde(rename = "deposit")]
    #[serde(borrow)]
    Deposit(Cow<'a, str>,),
    #[serde(rename = "resolve")]
    #[serde(borrow)]
    Resolve(Cow<'a, str>,),
    #[serde(rename = "withdrawal")]
    #[serde(borrow)]
    Withdraw(Cow<'a, str>,),
    #[serde(rename = "chargeback")]
    #[serde(borrow)]
    Chargeback(Cow<'a, str>,),
}

impl<'de, 'a> Deserialize<'de> for TransactionType<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "dispute" => Ok(TransactionType::Dispute(Cow::from(s))),
            "deposit" => Ok(TransactionType::Deposit(Cow::from(s))),
            "resolve" => Ok(TransactionType::Resolve(Cow::from(s))),
            "withdrawal" => Ok(TransactionType::Withdraw(Cow::from(s))),
            "chargeback" => Ok(TransactionType::Chargeback(Cow::from(s))),
            _ => Err(D::Error::custom("Failed to parse transaction.")),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Output {
    client: u16,
    available: f32,
    held: f32,
    total: f32,
    locked: bool,
}
