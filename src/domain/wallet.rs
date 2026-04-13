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

    pub fn deposit(&mut self, amount: Balance, currency: Currency) -> Result<(), String> {
        if self.currency != currency {
            return Err(format!(
                "Currency mismatch: cannot deposit {:?} into an {:?} wallet",
                currency, self.currency
            ));
        }

        self.balance = Balance::new(self.balance.value() + amount.value());

        Ok(())
    }

    /// Only allows withdrawal if currencies match AND funds are sufficient
    pub fn withdraw(&mut self, amount: Balance, currency: Currency) -> Result<(), String> {
        if self.currency != currency {
            return Err(format!(
                "Currency mismatch: cannot withdraw {:?} from an {:?} wallet",
                currency, self.currency
            ));
        }

        let current_val = self.balance.value();
        let withdraw_val = amount.value();

        if current_val < withdraw_val {
            return Err("Insufficient funds".to_string());
        }

        self.balance = Balance::new(current_val - withdraw_val);
        Ok(())
    }

    // Getter for wallets fixed currency
    pub fn currency(&self) -> &Currency {
        &self.currency
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

        wallet.deposit(Balance::new(489), Currency::XRP);

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

        let result = wallet.withdraw(Balance::new(50), Currency::XRP);

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
        let result = wallet.withdraw(Balance::new(150), Currency::XRP);

        // Assert that it failed and the balance stayed the same
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Insufficient funds");
        assert_eq!(wallet.balance(), 100);
    }
}
