use crate::traits::TransactionDB;
use crate::types::{Transaction, ClientAccount};
use std::sync::Arc;
use std::collections::HashMap;

pub struct MapAdapter {
    transactions_map: HashMap<u32, Transaction>,
    account_map: HashMap<u16, ClientAccount>,
}

impl MapAdapter {
    pub(crate) fn new() -> MapAdapter {
        MapAdapter {
            transactions_map: HashMap::new(),
            account_map: HashMap::new(),
        }
    }
}

impl TransactionDB for MapAdapter {
    fn add_account(&mut self, client_id: u16, client_account: ClientAccount) {
        self.account_map.insert(client_id, client_account);
    }

    fn add_transaction(&mut self, tx_id: u32, transaction: Transaction) {
        self.transactions_map.insert(tx_id, transaction);
    }

    fn get_transaction(&mut self, tx_id: u32) -> Option<&mut Transaction> {
        self.transactions_map.get_mut(&tx_id)
    }

    fn get_account(&mut self, account_id: u16) -> Option<&mut ClientAccount> {
        self.account_map.get_mut(&account_id)
    }

    fn display_all_accounts(&self) {
        self.account_map.values().for_each(|clients| println!("{:?}", clients));
    }
}