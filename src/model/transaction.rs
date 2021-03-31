use rust_decimal::Decimal;
use serde::Deserialize;

use super::{
    id::{client_id::ClientId, transaction_id::TransactionId},
    transaction_type::TransactionType,
};

#[derive(Debug, Deserialize, Clone)]
pub struct Transaction {
    #[serde(rename(deserialize = "type"))]
    pub transaction_type: TransactionType,
    pub client: ClientId,
    pub tx: TransactionId,
    pub amount: Option<Decimal>,
}
