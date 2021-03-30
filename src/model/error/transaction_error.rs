use rust_decimal::Decimal;
use thiserror::Error;

use crate::model::id::{client_id::ClientId, transaction_id::TransactionId};

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("Tried to withdraw {transaction_amount} from an available balance of {available} on client {client}")]
    Overdraft {
        client: ClientId,
        available: Decimal,
        transaction_amount: Decimal,
    },
    #[error("Transaction {0} does not exist")]
    TransactionDoesNotExist(TransactionId),
    #[error("Transaction {0} on client {1} either does not exist or is not disputed")]
    TransactionNotDisputed(TransactionId, ClientId),
    #[error("Account {0} is locked")]
    AccountLocked(ClientId),
}
