use crate::models::client::Client;
use crate::models::transaction::{InputRowTransactionType, Transaction};
use crate::types::ClientId;
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

        let client = client_db.entry(row.get_client()).or_insert(Client::new());

        match row.get_transaction_type() {
            InputRowTransactionType::Deposit => client.deposit(row.get_total()),
            InputRowTransactionType::Withdrawal => client.withdraw(row.get_total()),
            InputRowTransactionType::Dispute => client.dispute(row.get_total()),
            InputRowTransactionType::Resolve => client.resolve(row.get_total()),
            InputRowTransactionType::Chargeback => client.chargeback(row.get_total()),
        }
    }

    pub fn output_final_state(&self) -> Result<(), String> {
        let mut wtr = csv::Writer::from_writer(std::io::stdout());
        if let Err(err) = wtr.write_record(["client", "available", "held", "total"]) {
            return Err(err.to_string());
        }

        for (client_id, row) in self.client_db.iter() {
            let client = client_id.to_string();
            let available = row.get_available().to_string();
            let held = row.get_held().to_string();
            let total = row.get_total().to_string();

            if let Err(err) = wtr.write_record(&[client, available, held, total]) {
                return Err(err.to_string());
            }
        }

        if let Err(err) = wtr.flush() {
            return Err(err.to_string());
        }

        Ok(())
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
