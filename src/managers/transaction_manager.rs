use crate::common::types::{ClientId, TransactionId};
use crate::models::client::Client;
use crate::models::client_snapshot::ClientSnapshot;
use crate::models::transaction::{InputRowTransactionType, Transaction};
use std::collections::{HashMap, HashSet};

pub struct TransactionManager {
    client_db: HashMap<ClientId, Client>,
    tx_history: HashMap<(TransactionId, ClientId), Transaction>,
    disputed: HashSet<(TransactionId, ClientId)>,
}

impl TransactionManager {
    pub fn new() -> Self {
        TransactionManager {
            client_db: HashMap::new(),
            tx_history: HashMap::new(),
            disputed: HashSet::new(),
        }
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        let client_db = &mut self.client_db;

        let client_id = tx.get_client_id();
        let tx_id = tx.get_transaction_id();
        let tx_type = tx.get_transaction_type();
        let tx_amount = tx.get_amount();

        let client_txs = client_db.get(&client_id);
        if client_txs.is_none() {
            client_db.insert(client_id, Client::new(client_id));
        }
        let client = client_db.get_mut(&client_id).unwrap();
        if client.is_frozen() {
            // cannot do anything with frozen account
            return;
        }

        let id_pair = &(tx_id, client_id);

        match tx_type {
            InputRowTransactionType::Deposit => {
                // did transaction already happen?
                if self.tx_history.contains_key(id_pair) {
                    return;
                }

                // unwrap should be safe as we validated already
                client.deposit(tx_amount.unwrap());
                self.tx_history.insert(*id_pair, tx);
            }
            InputRowTransactionType::Withdrawal => {
                // did transaction already happen?
                if self.tx_history.contains_key(id_pair) {
                    return;
                }

                // unwrap should be safe as we validated already
                client.withdraw(tx_amount.unwrap());
                self.tx_history.insert(*id_pair, tx);
            }
            InputRowTransactionType::Dispute => {
                if let Some(transaction) = self.tx_history.get(id_pair) {
                    if let Some(amount) = transaction.get_net_amount() {
                        client.dispute(amount);
                        self.disputed.insert(*id_pair);
                    }
                }
            }
            InputRowTransactionType::Resolve => {
                // check if the transaction is disputed
                if !(self.disputed.contains(id_pair)) {
                    return;
                }

                if let Some(transaction) = self.tx_history.get(id_pair) {
                    if let Some(amount) = transaction.get_net_amount() {
                        client.resolve(amount);
                        self.disputed.remove(id_pair);
                    }
                }
            }
            InputRowTransactionType::Chargeback => {
                // check if the transaction is disputed
                if !(self.disputed.contains(id_pair)) {
                    return;
                }

                if let Some(transaction) = self.tx_history.get(id_pair) {
                    if let Some(amount) = transaction.get_net_amount() {
                        client.chargeback(amount);
                        client.freeze();
                        self.disputed.remove(id_pair);
                    }
                }
            }
        }
    }

    pub fn get_all_values(self) -> Vec<ClientSnapshot> {
        let snapshots = self
            .client_db
            .values()
            .map(|xx| xx.get_snapshot())
            .collect::<Vec<ClientSnapshot>>();

        snapshots
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn test_input_multiple_clients_deposit_withdraw() {
        // Arrange
        let mut transaction_manager = TransactionManager::new();

        let transaction1 = Transaction::new(
            1,
            InputRowTransactionType::Deposit,
            1,
            Some(Decimal::new(10, 1)),
        );
        let transaction2 = Transaction::new(
            2,
            InputRowTransactionType::Deposit,
            2,
            Some(Decimal::new(20, 1)),
        );
        let transaction3 = Transaction::new(
            3,
            InputRowTransactionType::Deposit,
            1,
            Some(Decimal::new(20, 1)),
        );
        let transaction4 = Transaction::new(
            4,
            InputRowTransactionType::Withdrawal,
            1,
            Some(Decimal::new(15, 1)),
        );
        let transaction5 = Transaction::new(
            5,
            InputRowTransactionType::Withdrawal,
            2,
            Some(Decimal::new(30, 1)),
        );

        // Act
        transaction_manager.add_transaction(transaction1);
        transaction_manager.add_transaction(transaction2);
        transaction_manager.add_transaction(transaction3);
        transaction_manager.add_transaction(transaction4);
        transaction_manager.add_transaction(transaction5);

        // Assert
        let client_db = transaction_manager.client_db;
        assert_eq!(client_db.len(), 2);

        let client_01 = client_db.get(&1).unwrap().get_snapshot();
        let client_02 = client_db.get(&2).unwrap().get_snapshot();

        assert_eq!(client_01.get_available(), Decimal::new(15, 1));
        assert_eq!(client_01.get_held(), Decimal::ZERO);
        assert_eq!(client_01.get_total(), Decimal::new(15, 1));

        assert_eq!(client_02.get_available(), Decimal::new(-10, 1));
        assert_eq!(client_02.get_held(), Decimal::ZERO);
        assert_eq!(client_02.get_total(), Decimal::new(-10, 1));
    }
}
