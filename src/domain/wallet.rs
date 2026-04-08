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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit_increases_balance() {
        // Setup the initial wallet state
        let mut wallet = Wallet::new(
            "rPT1Sjq2YGrBMTU2C27uPHqc9S7fPfMqM7",
            Balance::new(100),
            Currency::XRP,
        )
        .unwrap();

        wallet.deposit(Balance::new(489));

        assert_eq!(wallet.balance(), 589);
    }

    #[test]
    fn test_withdraw_decreases_balance() {
        // Setup the initial wallet state
        let mut wallet = Wallet::new(
            "rPT1Sjq2YGrBMTU2C27uPHqc9S7fPfMqM7",
            Balance::new(100),
            Currency::XRP,
        )
        .unwrap();

        let result = wallet.withdraw(Balance::new(50));

        assert!(result.is_ok());
        assert_eq!(wallet.balance(), 50);
    }

    #[test]
    fn test_withdraw_fails_if_insufficient_funds() {
        let mut wallet = Wallet::new(
            "rPT1Sjq2YGrBMTU2C27uPHqc9S7fPfMqM7",
            Balance::new(100),
            Currency::XRP,
        )
        .unwrap();

        // Attempting to withdraw more than we have
        let result = wallet.withdraw(Balance::new(150));

        // Assert that it failed and the balance stayed the same
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Insufficient funds");
        assert_eq!(wallet.balance(), 100);
    }
}
