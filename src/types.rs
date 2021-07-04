use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientAccount {
    pub(crate) client: u16,
    pub available: f32,
    pub held: f32,
    pub total: f32,
    pub locked: bool,
}

impl ClientAccount {
    pub fn into_formatted_f32(mut self) -> Self {
        self.available = format!("{:.4}", self.available).parse().unwrap();
        self.held = format!("{:.4}", self.held).parse().unwrap();
        self.total = format!("{:.4}", self.total).parse().unwrap();
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    pub client: u16,
    pub tx: u32,
    #[serde(default = "default_resource")]
    pub amount: Option<f32>,
}

fn default_resource() -> Option<f32> {
    Some(0.0000)
}

pub enum Action {
    NewTransaction(Transaction),
    DisplayTransactionFinished,
    DisplayTransaction,
}

#[derive(Debug, Serialize, Clone)]
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
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "dispute" => Ok(TransactionType::Dispute),
            "deposit" => Ok(TransactionType::Deposit),
            "resolve" => Ok(TransactionType::Resolve),
            "withdrawal" => Ok(TransactionType::Withdrawal),
            "chargeback" => Ok(TransactionType::Chargeback),
            _ => Err(serde::de::Error::custom("Failed to parse transaction.")),
        }
    }
}