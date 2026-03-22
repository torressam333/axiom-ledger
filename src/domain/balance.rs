use std::fmt;
use std::ops::Add;

// 1. Format XRP for humans and Drops for computers
impl fmt::Display for Balance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_drops = self.value();
        let xrp = total_drops / 1_000_000;
        let drops = total_drops % 1_000_000;

        if drops == 0 {
            write!(f, "{}", xrp)
        } else {
            // Ensure leading zeros are mainitained
            write!(f, "{}.{:06}", xrp, drops)
        }
    }
}

impl Add for Balance {
    type Output = Result<Self, String>;

    fn add(self, rhs: Self) -> Self::Output {
        self.value()
            .checked_add(rhs.value())
            .map(Self::new)
            .ok_or_else(|| "Balance overflow detected".to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Drops(pub u128);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Balance(Drops);

impl Balance {
    pub fn new(value: u128) -> Self {
        // u ints cant be negative but might add max limit in future
        Self(Drops(value))
    }

    pub fn value(&self) -> u128 {
        // Okayy so now we access the inner value of the new wrapped type.
        // Russian nesting dolls style lol
        (self.0).0
    }

    // Take a string and convert to xrp drops equivalent
    // ...into exact balance in drops (Whole and fractional XRP)
    pub fn from_xrp(amount: &str) -> Result<Self, String> {
        // Account for user copy-pasta amounts with whitespace
        let amount = amount.trim();
        let parts: Vec<&str> = amount.split(".").collect();

        match parts.len() {
            1 => {
                // No decimals i..e 100 -> catch invalid format
                let xrp: u128 = parts[0].parse().map_err(|_| "Invalid number format")?;

                Ok(Self::new(xrp * 1_000_000))
            }
            2 => {
                // Has a decimal like 1.5 to begin with
                let xrp: u128 = parts[0].parse().map_err(|_| "Invalid XRP part")?;
                let mut fraction_string = parts[1].to_string();

                if fraction_string.len() > 6 {
                    return Err("XRP precision cannot exceed 6 decimal places (1 drop)".into());
                }

                // Pad the string so that ".5" becomes "500000"
                while fraction_string.len() < 6 {
                    fraction_string.push('0');
                }

                let drops: u128 = fraction_string.parse().map_err(|_| "Invalid drop part")?;

                Ok(Self::new((xrp * 1_000_000) + drops))
            }
            _ => Err("Invalid format. Use '1.5' or '100'".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Balance;

    #[test]
    fn test_balance_display_formatting() {
        let test_cases = vec![
            (1_000_000, "1", "Whole XRP"),
            (1_500_000, "1.500000", "Standard decimal"),
            (1, "0.000001", "The Minimum Drop case"),
            (100, "0.000100", "Leading zeros in fraction"),
        ];

        for (drops, expected, msg) in test_cases {
            let bal = Balance::new(drops);
            assert_eq!(format!("{}", bal), expected, "Failed: {}", msg);
        }
    }

    #[test]
    fn test_balance_addition() {
        let a = Balance::new(1_000_000); // One XRP
        let b = Balance::new(500_000); // Half an xrp

        let result = (a + b).expect("Addition should succeed");

        assert_eq!(result.value(), 1_500_000);
    }

    #[test]
    fn test_balance_addition_overflow() {
        let a = Balance::new(u128::MAX);
        let b = Balance::new(1);

        let result = a + b; // Safelt addding bc of newtype :)

        assert!(result.is_err(), "Should have detected overflow");
    }
}
