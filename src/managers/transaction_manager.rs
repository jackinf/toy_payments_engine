use crate::common::types::ClientId;
use crate::models::client::Client;
use crate::models::transaction::{InputRowTransactionType, Transaction};
use std::collections::HashMap;

pub struct TransactionManager {
    client_db: HashMap<ClientId, Client>,
}

impl TransactionManager {
    pub fn new() -> Self {
        TransactionManager {
            client_db: HashMap::new(),
        }
    }

    pub fn add_transaction(&mut self, row: Transaction) {
        let client_db = &mut self.client_db;
        let client_id = row.get_client();

        let client = client_db.entry(client_id).or_insert(Client::new(client_id));

        match row.get_transaction_type() {
            InputRowTransactionType::Deposit => client.deposit(row.get_total()),
            InputRowTransactionType::Withdrawal => client.withdraw(row.get_total()),
            InputRowTransactionType::Dispute => client.dispute(row.get_total()),
            InputRowTransactionType::Resolve => client.resolve(row.get_total()),
            InputRowTransactionType::Chargeback => client.chargeback(row.get_total()),
        }
    }

    pub fn get_all_values(self) -> Vec<Client> {
        self.client_db.values().cloned().collect()
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

        let transaction1 =
            Transaction::new(InputRowTransactionType::Deposit, 1, 1, Decimal::new(10, 1));
        let transaction2 =
            Transaction::new(InputRowTransactionType::Deposit, 2, 2, Decimal::new(20, 1));
        let transaction3 =
            Transaction::new(InputRowTransactionType::Deposit, 1, 3, Decimal::new(20, 1));
        let transaction4 = Transaction::new(
            InputRowTransactionType::Withdrawal,
            1,
            4,
            Decimal::new(15, 1),
        );
        let transaction5 = Transaction::new(
            InputRowTransactionType::Withdrawal,
            2,
            5,
            Decimal::new(30, 1),
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

        let client_01 = client_db.get(&1).unwrap();
        let client_02 = client_db.get(&2).unwrap();

        assert_eq!(client_01.get_available(), Decimal::new(15, 1));
        assert_eq!(client_01.get_held(), Decimal::ZERO);
        assert_eq!(client_01.get_total(), Decimal::new(15, 1));

        assert_eq!(client_02.get_available(), Decimal::new(-10, 1));
        assert_eq!(client_02.get_held(), Decimal::ZERO);
        assert_eq!(client_02.get_total(), Decimal::new(-10, 1));
    }
}
