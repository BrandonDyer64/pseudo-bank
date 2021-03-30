use rust_decimal::Decimal;
use serde::Deserialize;

use super::id::{client_id::ClientId, transaction_id::TransactionId};

#[derive(Debug, Deserialize, Clone)]
pub struct Transaction {
    #[serde(rename(deserialize = "type"))]
    pub transaction_type: TransactionType,
    pub client: ClientId,
    pub tx: TransactionId,
    pub amount: Option<Decimal>,
}

#[derive(Debug, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TransactionType {
    Deposit,
    Withdraw,
    Dispute,
    Resolve,
    Chargeback,
}
