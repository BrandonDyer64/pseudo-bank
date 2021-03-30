use std::collections::HashMap;

use crate::model::{
    account::Account,
    id::{client_id::ClientId, transaction_id::TransactionId},
    transaction::Transaction,
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
        let account = self
            .accounts
            .entry(transaction.client)
            .or_insert_with(|| Account::new(transaction.client));

        let transaction = account.apply_transaction(&self.transactions, transaction);

        if let Some(transaction) = transaction {
            self.transactions
                .insert(transaction.tx, transaction.clone());
        }
    }

    pub fn get_accounts(&self) -> &HashMap<ClientId, Account> {
        &self.accounts
    }

    pub fn get_transactions(&self) -> &HashMap<TransactionId, Transaction> {
        &self.transactions
    }
}
