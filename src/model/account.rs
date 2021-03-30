use crate::model::id::client_id::ClientId;
use rust_decimal::Decimal;
use serde::{ser::SerializeStruct, Serialize, Serializer};

use super::transaction::{Transaction, TransactionType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    id: ClientId,
    available: Decimal,
    held: Decimal,
    is_locked: bool,
}

impl Account {
    pub fn new(id: ClientId) -> Account {
        Account {
            id,
            available: 0.into(),
            held: 0.into(),
            is_locked: false,
        }
    }

    pub fn apply_transaction(
        &mut self,
        transaction: &Transaction,
        existing_transaction: Option<&Transaction>,
    ) {
        if self.is_locked {
            return;
        }
        match transaction.transaction_type {
            TransactionType::Deposit => {
                self.available += transaction.amount.unwrap_or(0.into());
            }
            TransactionType::Withdraw => {
                let amount = transaction.amount.unwrap_or(0.into());
                let new_balance = self.available - amount;
                if new_balance > 0.into() {
                    self.available = new_balance;
                }
            }
            TransactionType::Dispute => {
                let amount = existing_transaction.and_then(|transaction| transaction.amount);
                if let Some(amount) = amount {
                    self.available -= amount;
                    self.held += amount;
                }
            }
            TransactionType::Resolve => {
                let amount = existing_transaction.and_then(|transaction| transaction.amount);
                if let Some(amount) = amount {
                    self.available += amount;
                    self.held -= amount;
                }
            }
            TransactionType::Chargeback => {
                let amount = existing_transaction.and_then(|transaction| transaction.amount);
                if let Some(amount) = amount {
                    self.held -= amount;
                    self.is_locked = true;
                }
            }
        }
    }
}

impl Serialize for Account {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Account", 4)?;
        state.serialize_field("client", &self.id)?;
        state.serialize_field("available", &self.available)?;
        state.serialize_field("held", &self.held)?;
        state.serialize_field("total", &(self.available + self.held))?;
        state.serialize_field("locked", &self.is_locked)?;
        state.end()
    }
}
