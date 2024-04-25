use crate::common::types::ClientId;
use crate::common::types::TransactionType;
use crate::models::client_snapshot::ClientSnapshot;
use rust_decimal::Decimal;

pub struct Client {
    client_id: ClientId,
    available: Decimal,
    held: Decimal,
    locked: bool,
}

impl Client {
    pub fn new(client_id: ClientId) -> Self {
        Client {
            client_id,
            available: Decimal::ZERO,
            held: Decimal::ZERO,
            locked: false,
        }
    }

    pub fn get_available(&self) -> Decimal {
        self.available
    }

    #[cfg(test)]
    pub fn get_held(&self) -> Decimal {
        self.held
    }

    pub fn deposit(&mut self, amount: Decimal) {
        self.available += amount;
    }

    pub fn withdraw(&mut self, amount: Decimal) {
        self.available -= amount;
    }

    pub fn dispute(&mut self, amount: Decimal, tx_type: TransactionType) {
        match tx_type {
            TransactionType::Deposit => {
                self.available -= amount;
                self.held += amount;
            }
            TransactionType::Withdrawal => {
                self.held += amount;
            }
            _ => {}
        }
    }

    pub fn resolve(&mut self, amount: Decimal, tx_type: TransactionType) {
        match tx_type {
            TransactionType::Deposit => {
                self.available += amount; // resolve the transaction (cancel dispute)
                self.held -= amount; // release held amount
            }
            TransactionType::Withdrawal => {
                self.held -= amount; // release held amount
            }
            _ => {}
        }
    }

    pub fn chargeback(&mut self, amount: Decimal, tx_type: TransactionType) {
        match tx_type {
            TransactionType::Deposit => {
                // just release held amount. The amount is already deducted from available
                self.held -= amount;
            }
            TransactionType::Withdrawal => {
                self.held -= amount;
                self.available += amount;
            }
            _ => {}
        }
    }

    pub fn freeze(&mut self) {
        self.locked = true;
    }

    pub fn is_locked(&self) -> bool {
        self.locked
    }

    pub fn get_snapshot(&self) -> ClientSnapshot {
        ClientSnapshot::new(self.client_id, self.available, self.held, self.locked)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_transactions_new() {
        let client_id = 1;
        let client = Client::new(client_id);

        assert_eq!(client.client_id, client_id);
        assert_eq!(client.available, Decimal::ZERO);
        assert_eq!(client.held, Decimal::ZERO);
    }

    /*
       Deposit tests
    */

    #[test]
    fn test_client_transactions_resolve_disputed_deposit() {
        let client_id = 1;
        let mut client = Client::new(client_id);

        client.deposit(Decimal::new(200, 2));
        assert_eq!(client.available, Decimal::new(200, 2));
        assert_eq!(client.held, Decimal::ZERO);

        client.dispute(Decimal::new(200, 2), TransactionType::Deposit);
        assert_eq!(client.available, Decimal::ZERO);
        assert_eq!(client.held, Decimal::new(200, 2));

        // Act & Assert
        client.resolve(Decimal::new(200, 2), TransactionType::Deposit);
        assert_eq!(client.available, Decimal::new(200, 2));
        assert_eq!(client.held, Decimal::ZERO);
    }

    #[test]
    fn test_client_transactions_chargeback_disputed_deposit() {
        let client_id = 1;
        let mut client = Client::new(client_id);

        client.deposit(Decimal::new(200, 2));
        assert_eq!(client.available, Decimal::new(200, 2));
        assert_eq!(client.held, Decimal::ZERO);

        client.dispute(Decimal::new(200, 2), TransactionType::Deposit);
        assert_eq!(client.available, Decimal::ZERO);
        assert_eq!(client.held, Decimal::new(200, 2));

        // Act & Assert
        client.chargeback(Decimal::new(200, 2), TransactionType::Deposit);
        assert_eq!(client.available, Decimal::ZERO);
        assert_eq!(client.held, Decimal::ZERO);
    }

    /*
       Withdrawal tests
    */

    #[test]
    fn test_client_transactions_resolve_disputed_withdrawal() {
        // Arrange
        let client_id = 1;
        let mut client = Client::new(client_id);

        client.deposit(Decimal::new(200, 2)); // to avoid negative balance
        client.withdraw(Decimal::new(50, 2));
        assert_eq!(client.available, Decimal::new(150, 2));
        assert_eq!(client.held, Decimal::ZERO);

        client.dispute(Decimal::new(50, 2), TransactionType::Withdrawal);
        assert_eq!(client.available, Decimal::new(150, 2));
        assert_eq!(client.held, Decimal::new(50, 2));

        // Act &  Assert
        client.resolve(Decimal::new(50, 2), TransactionType::Withdrawal);
        assert_eq!(client.available, Decimal::new(150, 2));
        assert_eq!(client.held, Decimal::ZERO);
    }

    #[test]
    fn test_client_transactions_chargeback_disputed_withdrawal() {
        // Arrange
        let client_id = 1;
        let mut client = Client::new(client_id);

        client.deposit(Decimal::new(200, 2));
        client.withdraw(Decimal::new(50, 2));
        assert_eq!(client.available, Decimal::new(150, 2));
        assert_eq!(client.held, Decimal::ZERO);

        client.dispute(Decimal::new(50, 2), TransactionType::Withdrawal);
        assert_eq!(client.available, Decimal::new(150, 2));
        assert_eq!(client.held, Decimal::new(50, 2));

        // Act & Assert
        client.chargeback(Decimal::new(50, 2), TransactionType::Withdrawal);
        assert_eq!(client.available, Decimal::new(200, 2));
        assert_eq!(client.held, Decimal::ZERO);
    }
}
