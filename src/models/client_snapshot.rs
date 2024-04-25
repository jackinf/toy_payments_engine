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
    locked: bool,
}

impl ClientSnapshot {
    pub fn new(id: ClientId, available: Decimal, held: Decimal, locked: bool) -> Self {
        ClientSnapshot {
            id,
            available,
            held,
            locked,
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

    pub fn get_locked(&self) -> bool {
        self.locked
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;

    #[test]
    fn test_client_snapshot_new() {
        let id = 1;
        let available = Decimal::new(1000, 2);
        let held = Decimal::new(100, 2);

        let snapshot = ClientSnapshot::new(id, available, held, false);

        assert_eq!(snapshot.get_id(), id);
        assert_eq!(snapshot.get_available(), available);
        assert_eq!(snapshot.get_held(), held);
        assert_eq!(snapshot.get_total(), available + held);
        assert_eq!(snapshot.get_locked(), false);
    }
}
