use rust_decimal::Decimal;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransactionError {
    #[error("Tried to withdraw {transaction_amount} from an available balance of {available}")]
    Overdraft {
        available: Decimal,
        transaction_amount: Decimal,
    },
    #[error("Transaction does not exist")]
    TransactionDoesNotExist,
    #[error("Transaction either does not exist or is not disputed")]
    TransactionNotDisputed,
    #[error("Account is locked")]
    AccountLocked,
}
