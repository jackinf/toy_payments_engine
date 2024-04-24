use crate::common::types::ClientId;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct Client {
    id: ClientId,
    available: Decimal,
    held: Decimal,
    total: Decimal,
}

impl Client {
    pub fn new(id: ClientId) -> Self {
        Client {
            id,
            available: Decimal::ZERO,
            held: Decimal::ZERO,
            total: Decimal::ZERO,
        }
    }

    pub fn deposit(&mut self, amount: Decimal) {
        self.available += amount;
        self.total += amount;
    }

    pub fn withdraw(&mut self, amount: Decimal) {
        self.available -= amount;
        self.total -= amount;
    }

    pub fn dispute(&mut self, amount: Decimal) {
        self.available -= amount;
        self.held += amount;
    }

    pub fn resolve(&mut self, amount: Decimal) {
        self.available += amount;
        self.held -= amount;
    }

    pub fn chargeback(&mut self, amount: Decimal) {
        self.held -= amount;
        self.total -= amount;
    }

    pub fn get_id(&self) -> ClientId {
        self.id
    }

    pub fn get_available(&self) -> Decimal {
        self.available
    }

    pub fn get_held(&self) -> Decimal {
        self.held
    }

    pub fn get_total(&self) -> Decimal {
        self.total
    }
}
