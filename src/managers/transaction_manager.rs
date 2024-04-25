use crate::common::types::{ClientId, TransactionId, TransactionType};
use crate::models::client::Client;
use crate::models::client_snapshot::ClientSnapshot;
use crate::models::transaction::Transaction;
use std::collections::{HashMap, HashSet};

pub struct TransactionManager {
    client_db: HashMap<ClientId, Client>,
    tx_history: HashMap<(TransactionId, ClientId), Transaction>,
    tx_disputed: HashSet<(TransactionId, ClientId)>,
}

impl TransactionManager {
    pub fn new() -> Self {
        TransactionManager {
            client_db: HashMap::new(),
            tx_history: HashMap::new(),
            tx_disputed: HashSet::new(),
        }
    }

    pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), String> {
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
            return Err(format!("Client {} is frozen", client_id));
        }

        let id_pair = &(tx_id, client_id);

        match tx_type {
            TransactionType::Deposit => {
                // did transaction already happen?
                if self.tx_history.contains_key(id_pair) {
                    return Err(format!("Transaction {} already happened", tx_id));
                }

                if let Some(amount) = tx_amount {
                    client.deposit(amount);
                    self.tx_history.insert(*id_pair, tx);
                } else {
                    return Err(format!("Transaction {} has no amount", tx_id));
                }
            }
            TransactionType::Withdrawal => {
                // did transaction already happen?
                if self.tx_history.contains_key(id_pair) {
                    return Err(format!("Transaction {} already happened", tx_id));
                }

                // check if the client has enough funds
                if let Some(amount) = tx_amount {
                    if client.get_available() < amount {
                        return Err(format!("Client {} has insufficient funds", client_id));
                    }
                    client.withdraw(amount);
                    self.tx_history.insert(*id_pair, tx);
                } else {
                    return Err(format!("Transaction {} has no amount", tx_id));
                }
            }
            TransactionType::Dispute => {
                if let Some(transaction_to_dispute) = self.tx_history.get(id_pair) {
                    if let Some(amount) = transaction_to_dispute.get_amount() {
                        client.dispute(amount, transaction_to_dispute.get_transaction_type());
                        self.tx_disputed.insert(*id_pair);
                    } else {
                        return Err(format!("Transaction {} has no amount", tx_id));
                    }
                } else {
                    return Err(format!("Transaction {} not found", tx_id));
                }
            }
            TransactionType::Resolve => {
                // check if the transaction is disputed
                if !(self.tx_disputed.contains(id_pair)) {
                    return Err(format!("Transaction {} is not disputed", tx_id));
                }

                if let Some(transaction) = self.tx_history.get(id_pair) {
                    if let Some(amount) = transaction.get_amount() {
                        client.resolve(amount, transaction.get_transaction_type());
                        self.tx_disputed.remove(id_pair);
                    } else {
                        return Err(format!("Transaction {} has no amount", tx_id));
                    }
                } else {
                    return Err(format!("Transaction {} not found", tx_id));
                }
            }
            TransactionType::Chargeback => {
                // check if the transaction is disputed
                if !(self.tx_disputed.contains(id_pair)) {
                    return Err(format!("Transaction {} is not disputed", tx_id));
                }

                if let Some(transaction) = self.tx_history.get(id_pair) {
                    if let Some(amount) = transaction.get_amount() {
                        client.chargeback(amount, transaction.get_transaction_type());
                        client.freeze();
                        self.tx_disputed.remove(id_pair);
                    } else {
                        return Err(format!("Transaction {} has no amount", tx_id));
                    }
                } else {
                    return Err(format!("Transaction {} not found", tx_id));
                }
            }
        }

        Ok(())
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
    use crate::common::types::TransactionType::{Deposit, Dispute, Withdrawal};
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;
    use Transaction as Tx;

    #[test]
    fn test_input_multiple_clients_deposit_withdraw() {
        let mut manager = TransactionManager::new();

        let tx1 = Tx::new(1, Deposit, 1, Some(dec!(1.0)));
        let res1 = manager.add_transaction(tx1);
        assert_eq!(res1, Ok(()));
        assert_balance(manager.client_db.get(&1).unwrap(), dec!(1.0), dec!(0));

        let tx2 = Tx::new(2, Deposit, 2, Some(dec!(2.0)));
        let res2 = manager.add_transaction(tx2);
        assert_eq!(res2, Ok(()));
        assert_balance(manager.client_db.get(&2).unwrap(), dec!(2.0), dec!(0));

        let tx3 = Tx::new(3, Deposit, 1, Some(dec!(2.0)));
        let res3 = manager.add_transaction(tx3);
        assert_eq!(res3, Ok(()));
        assert_balance(manager.client_db.get(&1).unwrap(), dec!(3.0), dec!(0));

        let tx4 = Tx::new(4, Withdrawal, 1, Some(dec!(1.5)));
        let res4 = manager.add_transaction(tx4);
        assert_eq!(res4, Ok(()));
        assert_balance(manager.client_db.get(&1).unwrap(), dec!(1.5), dec!(0));

        let tx5 = Tx::new(5, Withdrawal, 2, Some(dec!(3.0)));
        let res5 = manager.add_transaction(tx5);
        assert_eq!(res5, Err("Client 2 has insufficient funds".into()));
        assert_balance(manager.client_db.get(&1).unwrap(), dec!(1.5), dec!(0));
    }

    fn assert_balance(client: &Client, available: Decimal, held: Decimal) {
        assert_eq!(client.get_available(), available);
        assert_eq!(client.get_held(), held);
    }
}
