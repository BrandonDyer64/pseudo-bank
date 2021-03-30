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
        let existing_transaction = self.get_transaction(transaction.tx).map(|t| t.clone());
        let account = self
            .accounts
            .entry(transaction.client)
            .or_insert_with(|| Account::new(transaction.client));
        if account.is_locked() {
            return;
        }

        match transaction.transaction_type {
            TransactionType::Deposit => {
                account.deposit(transaction.amount.unwrap_or(0.into()));
                self.transactions.insert(transaction.tx, transaction);
            }
            TransactionType::Withdraw => {
                account.withdraw(transaction.amount.unwrap_or(0.into()));
                self.transactions.insert(transaction.tx, transaction);
            }
            TransactionType::Dispute => {
                let amount = existing_transaction.and_then(|transaction| transaction.amount);
                if let Some(amount) = amount {
                    account.dispute(amount);
                }
            }
            TransactionType::Resolve => {
                let amount = existing_transaction.and_then(|transaction| transaction.amount);
                if let Some(amount) = amount {
                    account.resolve(amount);
                }
            }
            TransactionType::Chargeback => {
                let amount = existing_transaction.and_then(|transaction| transaction.amount);
                if let Some(amount) = amount {
                    account.chargeback(amount);
                }
            }
        }
    }

    pub fn get_account(&self, client: ClientId) -> Option<&Account> {
        self.accounts.get(&client)
    }

    pub fn get_accounts(&self) -> &HashMap<ClientId, Account> {
        &self.accounts
    }

    pub fn get_transaction(&self, transaction: TransactionId) -> Option<&Transaction> {
        self.transactions.get(&transaction)
    }
}
