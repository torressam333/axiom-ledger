// Balance NewTpy
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Balance(u128);

impl Balance {
    pub fn new(value: u128) -> Self {
        // u ints cant be negative but might add max limit in future
        Self(value)
    }

    pub fn value(&self) -> u128 {
        self.0
    }
}

// Address NewTyp
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
    balance: Balance,
    currency: Currency,
}

impl Wallet {
    pub fn new(addr_str: &str, balance: Balance, currency: Currency) -> Result<Self, String> {
        let address = Address::parse(addr_str.to_string())?;

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
