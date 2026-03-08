#[derive(Debug, PartialEq, Clone)]
pub struct Address(String);

impl Address {
    pub fn parse(s: String) -> Result<Self, String> {
        if !s.starts_with("r") {
            return Err(format!("{} is not a valid XRPL address", s));
        }

        Ok(Self(s))
    }

    pub fn as_str(&self) -> &str {
        &self.0 // Get inner value
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Currency {
    XRP,
}

#[derive(Debug)]
pub struct Wallet {
    address: Address,
    balance: u128,
    currency: Currency,
}

impl Wallet {
    pub fn new(addr_str: &str, balance: u128, currency: Currency) -> Result<Self, String> {
        let address = Address::parse(addr_str.to_string())?;

        Ok(Self {
            address,
            balance,
            currency,
        })
    }

    pub fn balance(&self) -> u128 {
        self.balance
    }

    pub fn address(&self) -> &str {
        &self.address.as_str()
    }
}
