use crate::common::types::ClientId;
use rust_decimal::Decimal;

/**
 * ClientSnapshot is a snapshot of a client's account at a point in time.
 */
#[derive(Debug, Clone)]
pub struct ClientSnapshot {
    id: ClientId,
    available: Decimal,
    held: Decimal,
}

impl ClientSnapshot {
    pub fn new(id: ClientId, available: Decimal, held: Decimal) -> Self {
        ClientSnapshot {
            id,
            available,
            held,
        }
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
        self.available + self.held
    }
}
