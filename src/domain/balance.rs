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
