#[derive(Debug, PartialEq, Clone)]
pub enum Currency {
    XRP,
}

#[derive(Debug)]
pub struct Wallet {
    address: String,
    balance: u128,
    currency: Currency,
}

impl Wallet {
    pub fn new(address: &str, balance: u128, currency: Currency) -> Self {
        Self {
            address: address.to_string(),
            balance,
            currency,
        }
    }

    pub fn balance(&self) -> u128 {
        self.balance
    }

    pub fn address(&self) -> &str {
        &self.address
    }
}
