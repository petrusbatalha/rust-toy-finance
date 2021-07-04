use crate::traits::{AccountDB, TransactionDB};
use crate::types::{ClientAccount, Transaction};
use csv::WriterBuilder;
use std::collections::HashMap;
use std::io;

pub struct MapAdapter {
    accounts: HashMap<u16, ClientAccount>,
    transactions: HashMap<u32, Transaction>,
    under_dispute: HashMap<u32, Transaction>,
}

impl MapAdapter {
    pub(crate) fn new() -> MapAdapter {
        let under_dispute = HashMap::new();
        let transactions = HashMap::new();
        let accounts = HashMap::new();

        MapAdapter {
            accounts,
            transactions,
            under_dispute,
        }
    }
}

impl AccountDB for MapAdapter {
    fn add_account(&mut self, client_id: u16, client_account: ClientAccount) {
        self.accounts.insert(client_id, client_account);
    }

    fn get_account(&mut self, client_id: u16) -> Option<ClientAccount> {
        self.accounts.get(&client_id).cloned()
    }

    fn display_all_accounts(&self) {
        let mut wtr = WriterBuilder::new()
            .has_headers(true)
            .from_writer(io::stdout());
        self.accounts.iter().for_each(|client_account| {
            wtr.serialize(client_account.1.clone().into_formatted_f32())
                .expect("Failed to write csv to stdout.")
        });
    }
}

impl TransactionDB for MapAdapter {
    fn add_transaction(&mut self, tx_id: u32, transaction: Transaction) {
        self.transactions.insert(tx_id, transaction);
    }

    fn remove_transaction_from_dispute(&mut self, tx_id: u32) {
        match self.under_dispute.remove(&tx_id) {
            Some(_) => {}
            None => {
                warn!("Failed to remove transaction from dispute");
            }
        };
    }

    fn add_transaction_under_dispute(&mut self, tx_id: u32, transaction: Transaction) {
        self.under_dispute.insert(tx_id, transaction);
    }

    fn get_transaction_under_dispute(&mut self, tx_id: u32) -> Option<Transaction> {
        self.under_dispute.get(&tx_id).cloned()
    }

    fn get_transaction(&mut self, tx_id: u32) -> Option<Transaction> {
        self.transactions.get(&tx_id).cloned()
    }
}
