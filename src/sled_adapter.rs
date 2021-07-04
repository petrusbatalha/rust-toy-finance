use crate::traits::TransactionDB;
use crate::types::{ClientAccount, Transaction};
use csv::WriterBuilder;
use sled_extensions::bincode::Tree;
use sled_extensions::{DbExt, Error};
use std::io;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub struct SledAdapter {
    accounts: Tree<ClientAccount>,
    transactions: Tree<Transaction>,
}

impl SledAdapter {
    pub(crate) fn new() -> SledAdapter {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();

        let sled_db = sled_extensions::Config::new()
            .temporary(true)
            .path(format!("./sled_data{}", rand_string))
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
        let id = tx_id.as_ne_bytes();
        match self.transactions.insert(id, transaction) {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to add transaction {}", e);
            }
        };
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
