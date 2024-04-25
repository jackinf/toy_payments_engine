use crate::common::types::{ClientId, TransactionId};
use crate::models::client_snapshot::ClientSnapshot;
use crate::models::transaction::Transaction;
use rust_decimal::Decimal;
use std::collections::{HashMap, HashSet};

pub struct ClientTransactions {
    client_id: ClientId,
    available: Decimal,
    held: Decimal,
    tx_history: HashMap<TransactionId, Transaction>,
    disputed: HashSet<TransactionId>,
}

impl ClientTransactions {
    pub fn new(client_id: ClientId) -> Self {
        ClientTransactions {
            client_id,
            available: Decimal::ZERO,
            held: Decimal::ZERO,
            tx_history: HashMap::new(),
            disputed: HashSet::new(),
        }
    }

    pub fn deposit(&mut self, amount: Decimal) {
        self.available += amount;
    }

    pub fn withdraw(&mut self, amount: Decimal) {
        self.available -= amount;
    }

    pub fn dispute(&mut self, tx_id: TransactionId) {
        let transaction = self.tx_history.get(&tx_id).unwrap();
        if let Some(amount) = transaction.get_amount() {
            self.available -= amount;
            self.held += amount;
            self.disputed.insert(tx_id);
        }
    }

    pub fn resolve(&mut self, tx_id: TransactionId) {
        // check if the transaction is disputed
        if !(self.disputed.contains(&tx_id)) {
            return;
        }

        let transaction = self.tx_history.get(&tx_id).unwrap();
        if let Some(amount) = transaction.get_amount() {
            self.available += amount;
            self.held -= amount;
            self.disputed.remove(&tx_id);
        }
    }

    pub fn chargeback(&mut self, tx_id: TransactionId) {
        // check if the transaction is disputed
        if !(self.disputed.contains(&tx_id)) {
            return;
        }

        let transaction = self.tx_history.get(&tx_id).unwrap();
        if let Some(amount) = transaction.get_amount() {
            self.available -= amount;
            self.held -= amount;
            self.disputed.remove(&tx_id);
        }
    }

    pub fn get_snapshot(&self) -> ClientSnapshot {
        ClientSnapshot::new(self.client_id, self.available, self.held)
    }
}
