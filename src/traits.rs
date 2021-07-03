use crate::types::{ClientAccount, Transaction};

pub trait TransactionHandler {
    fn resolve(&mut self, tx_id: u32);
    fn dispute(&mut self, tx_id: u32);
    fn chargeback(&mut self, tx_id: u32);
    fn deposit(&mut self, transaction: Transaction);
    fn withdrawal(&mut self, transaction: Transaction);
}

pub trait TransactionDB {
    type DbError;
    fn add_account(&mut self, client_id: u16, client_account: ClientAccount);
    fn add_transaction(&mut self, tx_id: u32, transaction: Transaction);
    fn get_transaction(&mut self, tx_id: u32) -> Option<Transaction>;
    fn get_account(&mut self, client_id: u16) -> Option<ClientAccount>;
    fn display_all_accounts(&self);
}
