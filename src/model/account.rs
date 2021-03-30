use std::collections::HashMap;

use rust_decimal::Decimal;
use serde::{ser::SerializeStruct, Serialize, Serializer};

use super::{
    id::{client_id::ClientId, transaction_id::TransactionId},
    transaction::{Transaction, TransactionType},
};

#[derive(Debug, Clone)]
pub struct Account {
    id: ClientId,
    balance: Decimal,
    is_locked: bool,
    disputes: Vec<Transaction>,
}

impl Account {
    pub fn new(id: ClientId) -> Account {
        Account {
            id,
            balance: 0.into(),
            is_locked: false,
            disputes: Vec::new(),
        }
    }

    pub fn get_available(&self) -> Decimal {
        self.balance - self.get_held()
    }

    pub fn get_held(&self) -> Decimal {
        self.disputes
            .iter()
            .fold(0.into(), |acc, t| acc + t.amount.unwrap_or(0.into()))
    }

    pub fn apply_transaction(
        &mut self,
        transaction_store: &HashMap<TransactionId, Transaction>,
        transaction: Transaction,
    ) -> Option<Transaction> {
        if self.is_locked {
            return None;
        }
        match transaction.transaction_type {
            TransactionType::Deposit => {
                self.balance += transaction.amount.unwrap_or(0.into());
                Some(transaction)
            }
            TransactionType::Withdraw => {
                let amount = transaction.amount.unwrap_or(0.into());
                let new_balance = self.balance - amount;
                if new_balance > 0.into() {
                    self.balance = new_balance;
                }
                Some(transaction)
            }
            TransactionType::Dispute => {
                if let Some(disputed_transaction) = transaction_store.get(&transaction.tx) {
                    if !self.disputes.iter().any(|t| t.tx == transaction.tx) {
                        self.disputes.push(disputed_transaction.clone());
                    }
                }
                None
            }
            TransactionType::Resolve => {
                self.disputes = self
                    .disputes
                    .drain(..)
                    .filter(|t| t.tx != transaction.tx)
                    .collect::<Vec<_>>();
                None
            }
            TransactionType::Chargeback => {
                if let Some(disputed_transaction_amount) = self
                    .disputes
                    .iter()
                    .find(|t| t.tx == transaction.tx)
                    .and_then(|t| t.amount)
                {
                    self.is_locked = true;
                    self.disputes = self
                        .disputes
                        .drain(..)
                        .filter(|t| t.tx != transaction.tx)
                        .collect::<Vec<_>>();
                    self.balance -= disputed_transaction_amount;
                }
                None
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
        state.serialize_field("available", &self.get_available())?;
        state.serialize_field("held", &self.get_held())?;
        state.serialize_field("total", &self.balance)?;
        state.serialize_field("locked", &self.is_locked)?;
        state.end()
    }
}
