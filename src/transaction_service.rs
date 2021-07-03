use crate::types::{Transaction, TransactionType, ClientAccount, Action};
use tokio::sync::mpsc::Receiver;
use std::option::Option::Some;
use crate::traits::{TransactionDB, TransactionHandler};

pub struct TransactionService<T> {
    pub(crate) db_adapter: T,
}

impl <T: 'static + TransactionDB + std::marker::Sync + Send> TransactionService<T> {
    pub async fn receive(mut self, mut rcv: Receiver<Action>) {
        loop {
            if let Some(transaction) = rcv.recv().await {
                match transaction {
                    Action::NewTransaction(transaction) => {
                        match transaction.transaction_type {
                            TransactionType::Dispute => {}
                            TransactionType::Deposit => {
                                self.deposit(transaction);
                            }
                            TransactionType::Resolve => {}
                            TransactionType::Withdrawal => {
                                self.withdrawal(transaction);
                            }
                            TransactionType::Chargeback => {}
                        }
                    }
                    Action::DisplayTransaction => {
                        self.db_adapter.display_all_accounts();
                    }
                }
            }
        }
    }
}

impl <T: 'static + TransactionDB + std::marker::Sync + Send> TransactionHandler for TransactionService<T> {
    fn resolve(&self, _tx_id: u32) {
        unimplemented!()
    }

    fn lock(&self, _tx_id: u32) {
        unimplemented!()
    }

    fn chargeback(&self, _tx_id: u32) {
        unimplemented!()
    }

    fn deposit(&mut self, transaction: Transaction) {
        let client_account = self.db_adapter.get_account(transaction.client);
        match client_account {
            None => {
                self.db_adapter.add_account(transaction.client.clone(), ClientAccount {
                    client: transaction.client,
                    available: transaction.amount.unwrap(),
                    held: 0.0,
                    total: transaction.amount.unwrap(),
                    locked: false
                });
            }
            Some(mut client_account) => {
                client_account.total += transaction.amount.unwrap();
                client_account.available += transaction.amount.unwrap();
                self.db_adapter.add_account(transaction.client, client_account);
            }
        }
    }

    fn withdrawal(&mut self, transaction: Transaction) {
        let client_account = self.db_adapter.get_account(transaction.client);
        match client_account {
            None => {}
            Some(mut client_account) => {
                if client_account.available >= transaction.amount.unwrap() {
                    client_account.total -= transaction.amount.unwrap();
                    client_account.available -= transaction.amount.unwrap();
                    self.db_adapter.add_account(transaction.client, client_account);
                }
            }
        }
    }
}