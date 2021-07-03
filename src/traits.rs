use crate::types::{Transaction, ClientAccount};

pub trait TransactionHandler {
    fn resolve(&self, tx_id: u32);
    fn lock(&self, tx_id: u32);
    fn chargeback(&self, tx_id: u32);
    fn deposit(&self, client_id: u16);
    fn withdrawal(&self, client_id: u16);
}

pub trait TransactionDB {
    type DbError;
    fn add_account(&mut self, client_id: u16, client_account: ClientAccount);
    fn add_transaction(&mut self, tx_id: u32, transaction: Transaction);
    fn get_transaction(&mut self, tx_id: u32) -> Option<Transaction>;
    fn get_account(&mut self, client_id: u16) -> Option<ClientAccount>;
    fn display_all_accounts(&self);
}

