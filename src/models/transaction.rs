use crate::common::types::{ClientId, TransactionId};
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
    tx_type: InputRowTransactionType,
    client_id: ClientId,
    tx_id: TransactionId,
    amount: Option<Decimal>,
}

impl Transaction {
    pub fn new(
        tx_type: InputRowTransactionType,
        client_id: ClientId,
        tx_id: TransactionId,
        amount: Option<Decimal>,
    ) -> Self {
        Transaction {
            tx_type,
            client_id,
            tx_id,
            amount,
        }
    }

    pub fn get_transaction_id(&self) -> TransactionId {
        self.tx_id
    }

    pub fn get_transaction_type(&self) -> InputRowTransactionType {
        self.tx_type.clone()
    }

    pub fn get_client_id(&self) -> ClientId {
        self.client_id
    }

    pub fn get_amount(&self) -> Option<Decimal> {
        self.amount
    }
}

const COL_TX_TYPE: usize = 0;
const COL_CLIENT_ID: usize = 1;
const COL_TX_ID: usize = 2;
const COL_AMOUNT: usize = 3;

impl TryFrom<StringRecord> for Transaction {
    type Error = String; // TODO: to validation custom error type

    fn try_from(value: StringRecord) -> Result<Self, Self::Error> {
        // there should be 4 columns in the input row
        if value.len() != 4 {
            return Err("Invalid number of columns. Columns: type, client, tx, amount".into());
        }

        let col_tx_type = value.get(COL_TX_TYPE).unwrap().trim();
        let col_client_id = value.get(COL_CLIENT_ID).unwrap().trim();
        let col_tx_id = value.get(COL_TX_ID).unwrap().trim();
        let col_amount = value.get(COL_AMOUNT).unwrap().trim();

        let transaction_type = match col_tx_type {
            "deposit" => InputRowTransactionType::Deposit,
            "withdrawal" => InputRowTransactionType::Withdrawal,
            "dispute" => InputRowTransactionType::Dispute,
            "resolve" => InputRowTransactionType::Resolve,
            "chargeback" => InputRowTransactionType::Chargeback,
            _ => return Err("Invalid transaction type".into()),
        };

        let client_id: ClientId = match col_client_id.parse() {
            Ok(client) => client,
            Err(_) => return Err("Invalid client id".into()),
        };

        let tx = match col_tx_id.parse::<TransactionId>() {
            Ok(tx) => tx,
            Err(_) => return Err("Invalid transaction id".into()),
        };

        // If one of these transaction types were specified, the amount should be empty.
        let is_no_amount_transaction_type = transaction_type == InputRowTransactionType::Dispute
            || transaction_type == InputRowTransactionType::Resolve
            || transaction_type == InputRowTransactionType::Chargeback;

        if is_no_amount_transaction_type {
            if !col_amount.is_empty() {
                return Err("An amount should be empty".into());
            }

            return Ok(Transaction::new(transaction_type, client_id, tx, None));
        }

        let amount = match Decimal::from_str(col_amount) {
            Ok(amount) => amount.round_dp(4),
            Err(_) => return Err("Invalid amount".into()),
        };

        Ok(Transaction::new(
            transaction_type,
            client_id,
            tx,
            Some(amount),
        ))
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
        Transaction::new(InputRowTransactionType::Deposit, 1, 1, Some(Decimal::new(100, 1)))
    )]
    #[case(
        vec!["withdrawal", "1", "1", "10.0"],
        Transaction::new(InputRowTransactionType::Withdrawal, 1, 1, Some(Decimal::new(100, 1)))
    )]
    #[case(
        vec!["dispute", "1", "1", ""],
        Transaction::new(InputRowTransactionType::Dispute, 1, 1, None)
    )]
    #[case(
        vec!["resolve", "1", "1", ""],
        Transaction::new(InputRowTransactionType::Resolve, 1, 1, None)
    )]
    #[case(
        vec!["chargeback", "1", "1", ""],
        Transaction::new(InputRowTransactionType::Chargeback, 1, 1, None)
    )]
    fn test_transaction_from_string_record(
        #[case] input_vec: Vec<&str>,
        #[case] expected: Transaction,
    ) {
        // Arrange
        let record = StringRecord::from(input_vec);

        // Act
        let transaction = Transaction::try_from(record).unwrap();

        // Assert
        assert_eq!(transaction, expected);
    }

    #[test]
    fn test_invalid_transaction_type() {
        // Arrange
        let record = StringRecord::from(vec!["invalid", "1", "1", "10.0"]);

        // Act
        let result = Transaction::try_from(record);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_number_of_columns() {
        // Arrange
        let record = StringRecord::from(vec!["deposit", "1", "1", "10.0", "extra"]);

        // Act
        let result = Transaction::try_from(record);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_client_id() {
        // Arrange
        let record = StringRecord::from(vec!["deposit", "invalid", "1", "10.0"]);

        // Act
        let result = Transaction::try_from(record);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_transaction_id() {
        // Arrange
        let record = StringRecord::from(vec!["deposit", "1", "invalid", "10.0"]);

        // Act
        let result = Transaction::try_from(record);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_amount() {
        // Arrange
        let record = StringRecord::from(vec!["deposit", "1", "1", "invalid"]);

        // Act
        let result = Transaction::try_from(record);

        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_no_amount_for_non_empty_amount_transaction_type() {
        // Arrange
        let record = StringRecord::from(vec!["dispute", "1", "1", "10.0"]);

        // Act
        let result = Transaction::try_from(record);

        // Assert
        assert!(result.is_err());
    }
}
