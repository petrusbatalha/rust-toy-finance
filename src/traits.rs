use crate::types::{ClientAccount, Transaction};

pub trait TransactionHandler {
    fn resolve(&mut self, transaction: Transaction);
    fn dispute(&mut self, transaction: Transaction);
    fn chargeback(&mut self, transaction: Transaction);
    fn deposit(&mut self, transaction: Transaction);
    fn withdrawal(&mut self, transaction: Transaction);
}

pub trait AccountDB {
    fn add_account(&mut self, client_id: u16, client_account: ClientAccount);
    fn get_account(&mut self, client_id: u16) -> Option<ClientAccount>;
    fn display_all_accounts(&self);
}

pub trait TransactionDB {
    fn add_transaction(&mut self, tx_id: u32, transaction: Transaction);
    fn remove_transaction_from_dispute(&mut self, tx_id: u32);
    fn add_transaction_under_dispute(&mut self, tx_id: u32, transaction: Transaction);
    fn get_transaction_under_dispute(&mut self, tx_id: u32) -> Option<Transaction>;
    fn get_transaction(&mut self, tx_id: u32) -> Option<Transaction>;
}
