use crate::common::types::ClientId;
use crate::models::client_snapshot::ClientSnapshot;
use rust_decimal::Decimal;

pub struct Client {
    client_id: ClientId,
    available: Decimal,
    held: Decimal,
}

impl Client {
    pub fn new(client_id: ClientId) -> Self {
        Client {
            client_id,
            available: Decimal::ZERO,
            held: Decimal::ZERO,
        }
    }

    pub fn deposit(&mut self, amount: Decimal) {
        self.available += amount;
    }

    pub fn withdraw(&mut self, amount: Decimal) {
        self.available -= amount;
    }

    pub fn dispute(&mut self, amount: Decimal) {
        self.available -= amount;
        self.held += amount;
    }

    pub fn resolve(&mut self, amount: Decimal) {
        self.available += amount; // resolve the transaction (cancel dispute)
        self.held -= amount; // release held amount
    }

    pub fn chargeback(&mut self, amount: Decimal) {
        self.available -= amount; // reverse the transaction
        self.held -= amount; // release held amount
    }

    pub fn get_snapshot(&self) -> ClientSnapshot {
        ClientSnapshot::new(self.client_id, self.available, self.held)
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

    #[test]
    fn test_client_transactions_deposit() {
        let client_id = 1;
        let mut client = Client::new(client_id);

        let amount = Decimal::new(100, 2);
        client.deposit(amount);

        assert_eq!(client.available, amount);
    }

    #[test]
    fn test_client_transactions_withdraw() {
        let client_id = 1;
        let mut client = Client::new(client_id);

        let amount = Decimal::new(100, 2);
        client.withdraw(amount);

        assert_eq!(client.available, -amount);
    }

    #[test]
    fn test_client_transactions_dispute() {
        let client_id = 1;
        let mut client = Client::new(client_id);

        client.deposit(Decimal::new(200, 2));
        client.withdraw(Decimal::new(50, 2));
        client.dispute(Decimal::new(200, 2));

        assert_eq!(client.available, Decimal::new(-50, 2));
    }

    #[test]
    fn test_client_transactions_resolve() {
        let client_id = 1;
        let mut client = Client::new(client_id);

        client.deposit(Decimal::new(200, 2));
        client.withdraw(Decimal::new(50, 2));
        client.dispute(Decimal::new(200, 2));
        client.resolve(Decimal::new(200, 2));

        assert_eq!(client.available, Decimal::new(150, 2));
        assert_eq!(client.held, Decimal::ZERO);
    }

    #[test]
    fn test_client_transactions_chargeback() {
        let client_id = 1;
        let mut client = Client::new(client_id);

        client.deposit(Decimal::new(200, 2));
        client.withdraw(Decimal::new(50, 2));
        client.dispute(Decimal::new(200, 2));
        client.chargeback(Decimal::new(200, 2));

        assert_eq!(client.available, Decimal::new(-250, 2));
        assert_eq!(client.held, Decimal::ZERO);
    }
}
