#[derive(Debug)]
pub enum LedgerError {
    DatabaseError(String),
    InsufficientFunds,
    InvalidCurrency,
    DuplicateRequest,
}
