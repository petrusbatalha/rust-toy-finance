use crate::traits::TransactionDB;
use crate::types::{Transaction, ClientAccount};
use std::sync::Arc;
use std::collections::HashMap;
use sled_extensions::{Db, DbExt};
use sled_extensions::bincode::Tree;

pub struct SledAdapter {
    accounts: Tree<ClientAccount>,
    transactions: Tree<Transaction>,
}

impl SledAdapter {
    pub(crate) fn new() -> SledAdapter {
        let sled_db = sled_extensions::Config::default()
            .path("./sled_data")
            .open().expect("Failed to open sled.");
        let transactions = sled_db.open_bincode_tree("transactions").unwrap();
        let accounts = sled_db.open_bincode_tree("accounts").unwrap();
        SledAdapter {
            transactions,
            accounts
        }
    }
}

impl TransactionDB for SledAdapter {
    type DbError = sled_extensions::Error;
    fn add_account(&mut self, client_id: u16, client_account: ClientAccount) {
        self.accounts.insert(client_id.as_ne_bytes(), client_account).unwrap();
    }

    fn add_transaction(&mut self, tx_id: u32, transaction: Transaction) {
        self.transactions.insert(tx_id.as_ne_bytes(), transaction).unwrap();
    }

    fn get_transaction(&mut self, tx_id: u32) -> Option<Transaction> {
        self.transactions.get(tx_id.as_ne_bytes()).unwrap()
    }

    fn get_account(&mut self, client_id: u16) -> Option<ClientAccount> {
        self.accounts.get(client_id.as_ne_bytes()).unwrap()
    }

    fn display_all_accounts(&self) {
        self.accounts.iter().values().for_each((|clients| println!("{:?}", clients)));
    }
}