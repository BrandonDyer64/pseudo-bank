use std::collections::HashMap;

use crate::model::{
    account::Account,
    id::{client_id::ClientId, transaction_id::TransactionId},
    transaction::{Transaction, TransactionType},
};

#[derive(Debug)]
pub struct Store {
    accounts: HashMap<ClientId, Account>,
    transactions: HashMap<TransactionId, Transaction>,
}

impl Store {
    pub fn new() -> Store {
        Store {
            accounts: HashMap::new(),
            transactions: HashMap::new(),
        }
    }

    pub fn apply_transaction(&mut self, transaction: Transaction) {
        let existing_transaction = self.transactions.get(&transaction.tx);
        let account = self
            .accounts
            .entry(transaction.client)
            .or_insert_with(|| Account::new(transaction.client));

        account.apply_transaction(&transaction, existing_transaction);

        match transaction.transaction_type {
            TransactionType::Deposit | TransactionType::Withdraw => {
                self.transactions
                    .insert(transaction.tx, transaction.clone());
            }
            _ => (),
        }
    }

    pub fn get_accounts(&self) -> &HashMap<ClientId, Account> {
        &self.accounts
    }
}
