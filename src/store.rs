use std::collections::HashMap;

use crate::{
    account::Account,
    transaction::{Transaction, TransactionType},
};

pub struct Store {
    accounts: HashMap<u16, Account>,
}

impl Store {
    pub fn apply_transaction(&mut self, transaction: Transaction) {
        let account = self
            .accounts
            .entry(transaction.client)
            .or_insert_with(|| Account::new());

        match transaction.transaction_type {
            TransactionType::Deposit => {
                account.deposit(transaction.amount.unwrap_or(0.into()));
            }
            TransactionType::Withdraw => {
                account.debit(transaction.amount.unwrap_or(0.into()));
            }
            _ => unimplemented!(),
        }
    }
}
