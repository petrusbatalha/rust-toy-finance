use crate::traits::TransactionDB;
use crate::types::{ClientAccount, Transaction};
use csv::WriterBuilder;
use sled_extensions::bincode::Tree;
use sled_extensions::DbExt;
use std::io;

pub struct SledAdapter {
    accounts: Tree<ClientAccount>,
    transactions: Tree<Transaction>,
}

impl SledAdapter {
    pub(crate) fn new() -> SledAdapter {
        let sled_db = sled_extensions::Config::new()
            .temporary(true)
            .path("./sled_data")
            .open()
            .expect("Failed to open sled.");
        let transactions = sled_db.open_bincode_tree("transactions").unwrap();
        let accounts = sled_db.open_bincode_tree("accounts").unwrap();
        SledAdapter {
            transactions,
            accounts,
        }
    }
}

impl TransactionDB for SledAdapter {
    type DbError = sled_extensions::Error;
    fn add_account(&mut self, client_id: u16, client_account: ClientAccount) {
        self.accounts
            .insert(client_id.as_ne_bytes(), client_account)
            .unwrap();
    }

    fn add_transaction(&mut self, tx_id: u32, transaction: Transaction) {
        self.transactions
            .insert(tx_id.as_ne_bytes(), transaction)
            .unwrap();
    }

    fn get_transaction(&mut self, tx_id: u32) -> Option<Transaction> {
        self.transactions.get(tx_id.as_ne_bytes()).unwrap()
    }

    fn get_account(&mut self, client_id: u16) -> Option<ClientAccount> {
        self.accounts.get(client_id.as_ne_bytes()).unwrap()
    }

    fn display_all_accounts(&self) {
        let mut wtr = WriterBuilder::new()
            .has_headers(true)
            .from_writer(io::stdout());
        self.accounts
            .iter()
            .values()
            .flat_map(|value| value.ok())
            .map(|client_account| client_account.into_formatted_f32())
            .for_each(|client_account| {
                wtr.serialize(client_account).expect("Failed to write csv to stdout.")
            });
    }
}
