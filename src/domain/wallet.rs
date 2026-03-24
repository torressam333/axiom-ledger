use std::str::FromStr;

pub use crate::domain::address::Address;
pub use crate::domain::balance::Balance;

#[derive(Debug, PartialEq, Clone)]
pub enum Currency {
    XRP,
}

#[derive(Debug)]
pub struct Wallet {
    address: Address,
    balance: Balance,
    currency: Currency,
}

impl Wallet {
    pub fn new(addr_str: &str, balance: Balance, currency: Currency) -> Result<Self, String> {
        let address = Address::from_str(addr_str)?;

        Ok(Self {
            address,
            balance,
            currency,
        })
    }

    pub fn balance(&self) -> u128 {
        self.balance.value()
    }

    pub fn address(&self) -> &str {
        self.address.as_str()
    }

    pub fn deposit(&mut self, amount: Balance) {
        self.balance = Balance::new(self.balance.value() + amount.value())
    }

    pub fn withdraw(&mut self, withdraw_amount: Balance) -> Result<(), String> {
        let actual_balance = self.balance.value();
        let withdraw_value = withdraw_amount.value();

        // Check if the with withdraw amt will leave user in negative state, if so deny it asap
        if actual_balance < withdraw_value {
            return Err(String::from("Insufficient funds"));
        }

        self.balance = Balance::new(self.balance.value() - withdraw_amount.value());

        Ok(())
    }
}
