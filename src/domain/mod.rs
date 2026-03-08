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
    pub fn new(address: &str, balance: u128, currency: Currency) -> Result<Self, String> {
        if !address.starts_with("r") {
            return Err(format!("{} is not a valid XRPL address", address));
        }

        Ok(Self {
            address: address.to_string(),
            balance,
            currency,
        })
    }

    pub fn balance(&self) -> u128 {
        self.balance
    }

    pub fn address(&self) -> &str {
        &self.address
    }
}
