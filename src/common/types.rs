pub type ClientId = u16;
pub type TransactionId = u32;

#[derive(Debug, PartialEq, Clone)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}
