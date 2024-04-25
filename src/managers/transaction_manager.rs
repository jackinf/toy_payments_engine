use crate::common::types::ClientId;
use crate::models::client_snapshot::ClientSnapshot;
use crate::models::client_transactions::ClientTransactions;
use crate::models::transaction::{InputRowTransactionType, Transaction};
use std::collections::HashMap;

pub struct TransactionManager {
    client_db: HashMap<ClientId, ClientTransactions>,
}

impl TransactionManager {
    pub fn new() -> Self {
        TransactionManager {
            client_db: HashMap::new(),
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
            client_db.insert(client_id, ClientTransactions::new(client_id));
        }
        let client_txs = client_db.get_mut(&client_id).unwrap();

        match tx_type {
            // unwrap should be safe as we validated already
            InputRowTransactionType::Deposit => client_txs.deposit(tx_amount.unwrap()),
            // unwrap should be safe as we validated already
            InputRowTransactionType::Withdrawal => client_txs.withdraw(tx_amount.unwrap()),
            InputRowTransactionType::Dispute => client_txs.dispute(tx_id),
            InputRowTransactionType::Resolve => client_txs.resolve(tx_id),
            InputRowTransactionType::Chargeback => client_txs.chargeback(tx_id),
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
    fn test_input_row_from_string_record() {
        // Arrange
        let mut transaction_manager = TransactionManager::new();

        let transaction1 = Transaction::new(
            InputRowTransactionType::Deposit,
            1,
            1,
            Some(Decimal::new(10, 1)),
        );
        let transaction2 = Transaction::new(
            InputRowTransactionType::Deposit,
            2,
            2,
            Some(Decimal::new(20, 1)),
        );
        let transaction3 = Transaction::new(
            InputRowTransactionType::Deposit,
            1,
            3,
            Some(Decimal::new(20, 1)),
        );
        let transaction4 = Transaction::new(
            InputRowTransactionType::Withdrawal,
            1,
            4,
            Some(Decimal::new(15, 1)),
        );
        let transaction5 = Transaction::new(
            InputRowTransactionType::Withdrawal,
            2,
            5,
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
