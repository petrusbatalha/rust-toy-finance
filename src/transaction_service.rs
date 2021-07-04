use crate::traits::{TransactionDB, TransactionHandler};
use crate::types::{Action, ClientAccount, Transaction, TransactionType};
use std::option::Option::Some;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct TransactionService<T> {
    pub(crate) db_adapter: T,
}

impl<T: 'static + TransactionDB + std::marker::Sync + Send> TransactionService<T> {
    pub async fn receive(mut self, mut rcv: Receiver<Action>, status_sender: Sender<Action>) {
        loop {
            while let Some(action) = rcv.recv().await {
                match action {
                        Action::NewTransaction(transaction) => match transaction.transaction_type {
                            TransactionType::Dispute => {
                                self.dispute(transaction.tx);
                            }
                            TransactionType::Deposit => {
                                self.deposit(transaction);
                            }
                            TransactionType::Resolve => {
                                self.resolve(transaction.tx);
                            }
                            TransactionType::Withdrawal => {
                                self.withdrawal(transaction);
                            }
                            TransactionType::Chargeback => {
                                self.chargeback(transaction.tx)
                            }
                        },
                    Action::DisplayTransaction => {
                        self.db_adapter.display_all_accounts();
                        status_sender.send(Action::DisplayTransactionFinished).await.ok();
                    }
                    _ => {}
                }
            }
        }
    }
}

impl<T: 'static + TransactionDB + std::marker::Sync + Send> TransactionHandler
    for TransactionService<T>
{
    fn resolve(&mut self, tx_id: u32) {
        match self.db_adapter.get_transaction(tx_id) {
            None => {}
            Some(mut transaction) => {
                match self.db_adapter.get_account(transaction.client) {
                    None => {}
                    Some(mut client_account) => {
                        if let Some(dispute) = transaction.dispute {
                            if dispute {
                                client_account.available += transaction.amount.unwrap();
                                client_account.held -= transaction.amount.unwrap();
                                transaction.dispute = Some(false);
                                self.db_adapter.add_account(client_account.client, client_account);
                                self.db_adapter.add_transaction(tx_id, transaction);
                            }
                        }
                    }
                };
            }
        };
    }

    fn dispute(&mut self, tx_id: u32) {
        match self.db_adapter.get_transaction(tx_id) {
            None => {}
            Some(mut transaction) => {
                match self.db_adapter.get_account(transaction.client) {
                    None => {}
                    Some(mut client_account) => {
                        if let Some(dispute) = transaction.dispute {
                            if !dispute {
                                client_account.available -= transaction.amount.unwrap();
                                client_account.held += transaction.amount.unwrap();
                                transaction.dispute = Some(true);
                                self.db_adapter.add_account(client_account.client, client_account);
                                self.db_adapter.add_transaction(tx_id, transaction);
                            }
                        }
                    }
                };
            }
        };
    }

    fn chargeback(&mut self, tx_id: u32) {
        match self.db_adapter.get_transaction(tx_id) {
            None => {}
            Some(mut transaction) => {
                match self.db_adapter.get_account(transaction.client) {
                    None => {}
                    Some(mut client_account) => {
                        if let Some(dispute) = transaction.dispute {
                            if dispute {
                                client_account.held -= transaction.amount.unwrap();
                                client_account.total -= transaction.amount.unwrap();
                                client_account.locked = true;
                                transaction.dispute = Some(false);
                                self.db_adapter.add_account(client_account.client, client_account);
                                self.db_adapter.add_transaction(tx_id, transaction);
                            }
                        }
                    }
                };
            }
        };
    }

    fn deposit(&mut self, transaction: Transaction) {
        let client_account = self.db_adapter.get_account(transaction.client);
        match client_account {
            None => {
                self.db_adapter.add_account(
                    transaction.client.clone(),
                    ClientAccount {
                        client: transaction.client,
                        available: transaction.amount.unwrap(),
                        held: 0.0,
                        total: transaction.amount.unwrap(),
                        locked: false,
                    },
                );
            }
            Some(mut client_account) => {
                if client_account.locked == false {
                    client_account.total += transaction.amount.unwrap();
                    client_account.available += transaction.amount.unwrap();
                    self.db_adapter.add_account(transaction.client, client_account);
                }
            }
        }
        self.db_adapter.add_transaction(transaction.tx, transaction);
    }

    fn withdrawal(&mut self, transaction: Transaction) {
        let client_account = self.db_adapter.get_account(transaction.client);
        match client_account {
            None => {}
            Some(mut client_account) => {
                if client_account.locked == false
                    && client_account.available >= transaction.amount.unwrap() {
                    client_account.total -= transaction.amount.unwrap();
                    client_account.available -= transaction.amount.unwrap();
                    self.db_adapter.add_account(transaction.client, client_account);
                    self.db_adapter.add_transaction(transaction.tx, transaction);
                }
            }
        }
    }
}
