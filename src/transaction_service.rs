use crate::types::{Transaction, TransactionType, ClientAccount};
use tokio::sync::mpsc::Receiver;
use std::option::Option::Some;
use crate::traits::TransactionDB;

pub struct TransactionService<T> {
    pub(crate) db_adapter: T,
}

impl <T: 'static + TransactionDB + std::marker::Sync + Send> TransactionService<T> {
    pub async fn receive(mut self, mut rcv: Receiver<Transaction>) {
        loop {
            if let Some(transaction) = rcv.recv().await {
                match transaction.transaction_type {
                    TransactionType::Dispute => {}
                    TransactionType::Deposit => {
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
                            }
                        }
                    }
                    TransactionType::Resolve => {}
                    TransactionType::Withdrawal => {
                        let client_account =
                            self.db_adapter.get_account(transaction.client);
                        match client_account {
                            None => {},
                            Some(mut client_account) => {
                                if client_account.available >= transaction.amount.unwrap() {
                                    client_account.total -= transaction.amount.unwrap();
                                    client_account.available -= transaction.amount.unwrap();
                                }
                            }
                        };
                    }
                    TransactionType::Chargeback => {}
                }
            }
        }
    }
    pub fn display_all_accounts(&self) {
        self.db_adapter.display_all_accounts();
    }
}