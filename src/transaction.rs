use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename(deserialize = "type"))]
    pub transaction_type: TransactionType,
    pub client: u16,
    pub tx: u32,
    pub amount: Option<Decimal>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdraw,
    Dispute,
    Resolve,
    Chargeback,
}
