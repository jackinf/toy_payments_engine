use crate::types::{ClientId, TransactionId};
use csv::StringRecord;
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum InputRowTransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Debug, PartialEq)]
pub struct Transaction {
    transaction_type: InputRowTransactionType,
    client: ClientId,
    tx: TransactionId,
    available: Decimal,
    held: Decimal,
    total: Decimal,
}

impl Transaction {
    pub fn new(
        transaction_type: InputRowTransactionType,
        client: ClientId,
        tx: TransactionId,
        amount: Decimal,
    ) -> Self {
        Transaction {
            transaction_type,
            client,
            tx,
            available: amount, // decimal uses Copy trait
            held: Decimal::ZERO,
            total: amount,
        }
    }

    pub fn get_transaction_type(&self) -> InputRowTransactionType {
        self.transaction_type.clone()
    }

    pub fn get_client(&self) -> ClientId {
        self.client
    }

    pub fn get_total(&self) -> Decimal {
        self.total
    }
}

impl From<StringRecord> for Transaction {
    fn from(row: StringRecord) -> Self {
        let transaction_type = match row.get(0).unwrap() {
            "deposit" => InputRowTransactionType::Deposit,
            "withdrawal" => InputRowTransactionType::Withdrawal,
            "dispute" => InputRowTransactionType::Dispute,
            "resolve" => InputRowTransactionType::Resolve,
            "chargeback" => InputRowTransactionType::Chargeback,
            _ => panic!("Invalid transaction type"),
        };

        let client = row.get(1).unwrap().parse::<ClientId>().unwrap();
        let tx = row.get(2).unwrap().parse::<TransactionId>().unwrap();
        let amount = Decimal::from_str(row.get(3).unwrap()).unwrap();

        Transaction::new(transaction_type, client, tx, amount)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use csv::StringRecord;
    use rstest::rstest;
    use rust_decimal::Decimal;

    #[rstest]
    #[case(
        vec!["deposit", "1", "1", "10.0"],
        Transaction::new(InputRowTransactionType::Deposit, 1, 1, Decimal::new(100, 1))
    )]
    #[case(
        vec!["withdrawal", "1", "1", "10.0"],
        Transaction::new(InputRowTransactionType::Withdrawal, 1, 1, Decimal::new(100, 1))
    )]
    #[case(
        vec!["dispute", "1", "1", "10.0"],
        Transaction::new(InputRowTransactionType::Dispute, 1, 1, Decimal::new(100, 1))
    )]
    #[case(
        vec!["resolve", "1", "1", "10.0"],
        Transaction::new(InputRowTransactionType::Resolve, 1, 1, Decimal::new(100, 1))
    )]
    #[case(
        vec!["chargeback", "1", "1", "10.0"],
        Transaction::new(InputRowTransactionType::Chargeback, 1, 1, Decimal::new(100, 1))
    )]
    fn test_transaction_from_string_record(
        #[case] input_vec: Vec<&str>,
        #[case] expected: Transaction,
    ) {
        // Arrange
        let record = StringRecord::from(input_vec);

        // Act
        let transaction = Transaction::from(record);

        // Assert
        assert_eq!(transaction, expected);
    }
}
