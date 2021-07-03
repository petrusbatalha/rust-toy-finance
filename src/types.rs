use serde::{Serialize, Deserialize, Deserializer};
use serde::de::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientAccount {
    pub(crate) client: u16,
    pub available: f32,
    pub held: f32,
    pub total: f32,
    pub locked: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<f32>
}

#[derive(Debug, Serialize)]
pub enum TransactionType {
    #[serde(rename = "dispute")]
    Dispute,
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "resolve")]
    Resolve,
    #[serde(rename = "withdrawal")]
    Withdrawal,
    #[serde(rename = "chargeback")]
    Chargeback,
}

impl<'de> Deserialize<'de> for TransactionType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "dispute" => Ok(TransactionType::Dispute),
            "deposit" => Ok(TransactionType::Deposit),
            "resolve" => Ok(TransactionType::Resolve),
            "withdrawal" => Ok(TransactionType::Withdrawal),
            "chargeback" => Ok(TransactionType::Chargeback),
            _ => Err(D::Error::custom("Failed to parse transaction.")),
        }
    }
}