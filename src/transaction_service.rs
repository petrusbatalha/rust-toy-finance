use crate::traits::{AccountDB, TransactionDB, TransactionHandler};
use crate::types::{Action, ClientAccount, Transaction, TransactionType};
use std::option::Option::Some;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct TransactionService<T> {
    pub(crate) db_adapter: T,
}

impl<T: 'static + TransactionDB + AccountDB> TransactionService<T> {
    pub async fn receive(mut self, mut rcv: Receiver<Action>, status_sender: Sender<Action>) {
        loop {
            while let Some(action) = rcv.recv().await {
                match action {
                    Action::NewTransaction(transaction) => match transaction.transaction_type {
                        TransactionType::Dispute => self.dispute(transaction),
                        TransactionType::Deposit => self.deposit(transaction),
                        TransactionType::Resolve => self.resolve(transaction),
                        TransactionType::Withdrawal => self.withdrawal(transaction),
                        TransactionType::Chargeback => self.chargeback(transaction),
                    },
                    Action::DisplayTransaction => {
                        self.db_adapter.display_all_accounts();
                        status_sender
                            .send(Action::DisplayTransactionFinished)
                            .await
                            .ok();
                    }
                    Action::DisplayTransactionFinished => {}
                }
            }
        }
    }
}

impl<T: 'static + TransactionDB + AccountDB> TransactionHandler for TransactionService<T> {
    fn resolve(&mut self, transaction: Transaction) {
        let id = transaction.tx;
        if let Some(dispute) = self.db_adapter.get_transaction_under_dispute(id) {
            let mut client_account = self.db_adapter.get_account(dispute.client).unwrap();
            client_account.available += dispute.amount.unwrap();
            client_account.held -= dispute.amount.unwrap();
            self.db_adapter
                .add_account(client_account.client, client_account);
            self.db_adapter.remove_transaction_from_dispute(id);
        }
    }

    fn dispute(&mut self, transaction: Transaction) {
        let id = transaction.tx;
        match self.db_adapter.get_transaction(id) {
            None => {}
            Some(transaction_to_be_disputed) => {
                let mut client_account = self
                    .db_adapter
                    .get_account(transaction_to_be_disputed.client)
                    .unwrap();
                client_account.available -= transaction_to_be_disputed.amount.unwrap();
                client_account.held += transaction_to_be_disputed.amount.unwrap();
                self.db_adapter
                    .add_account(client_account.client, client_account);
                self.db_adapter
                    .add_transaction_under_dispute(id, transaction_to_be_disputed);
            }
        }
    }

    fn chargeback(&mut self, transaction: Transaction) {
        let id = transaction.tx;
        if let Some(dispute) = self.db_adapter.get_transaction_under_dispute(id) {
            match self.db_adapter.get_account(transaction.client) {
                None => {}
                Some(mut client_account) => {
                    client_account.held -= dispute.amount.unwrap();
                    client_account.total -= dispute.amount.unwrap();
                    client_account.locked = true;
                    self.db_adapter
                        .add_account(client_account.client, client_account);
                    self.db_adapter.remove_transaction_from_dispute(id)
                }
            }
        }
    }

    fn deposit(&mut self, transaction: Transaction) {
        let client_account = self.db_adapter.get_account(transaction.client);
        match client_account {
            None => {
                self.db_adapter.add_account(
                    transaction.client,
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
                if !client_account.locked {
                    client_account.total += transaction.amount.unwrap();
                    client_account.available += transaction.amount.unwrap();
                    self.db_adapter
                        .add_account(transaction.client, client_account);
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
                if !client_account.locked && client_account.available >= transaction.amount.unwrap()
                {
                    client_account.total -= transaction.amount.unwrap();
                    client_account.available -= transaction.amount.unwrap();
                    self.db_adapter
                        .add_account(transaction.client, client_account);
                    self.db_adapter.add_transaction(transaction.tx, transaction);
                }
            }
        }
    }
}
