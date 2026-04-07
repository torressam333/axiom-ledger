use core::fmt;
use std::format;
use std::str::FromStr;
use validator::{Validate, ValidationError};

// Address NewType
#[derive(Debug, PartialEq, Clone, Validate)]
pub struct Address {
    // Give the field a name: 'value'
    #[validate(custom(function = "validate_xrpl_address"))]
    pub value: String,
}

impl Address {
    pub fn new(s: String) -> Result<Self, String> {
        let addr = Self { value: s };

        addr.validate()
            .map(|_| addr)
            .map_err(|e| format!("Address validation failed {}", e))
    }

    // Allow other parts to borrow inner string for reaing
    pub fn as_str(&self) -> &str {
        &self.value
    }
}

// Need a way to turn address -> String
impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

// No custom parse, using idiomatic way to handle str parsing
// Turns string -> Address
impl FromStr for Address {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

fn validate_xrpl_address(address: &str) -> Result<(), ValidationError> {
    if !address.starts_with('r') {
        return Err(ValidationError::new("must_start_with_r"));
    }

    if address.len() < 25 || address.len() > 35 {
        return Err(ValidationError::new("invalid_length"));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_as_str_accessor() {
        let raw = "rPT1Sjq2YGrBMTU2C27uPHqc9S7fPfMqM7".to_string();
        let addr = Address::new(raw.clone()).unwrap();
        // Verify we can "view" the string without moving it
        assert_eq!(addr.as_str(), raw);
    }

    #[test]
    fn test_address_from_str_trait() {
        let raw = "rPT1Sjq2YGrBMTU2C27uPHqc9S7fPfMqM7";
        // This tests that .parse() now works automatically
        let addr: Result<Address, String> = raw.parse();
        assert!(addr.is_ok());
        assert_eq!(addr.unwrap().value, raw);
    }

    #[test]
    fn test_validation_rules() {
        let test_cases = vec![
            (
                "rPT1Sjq2YGrBMTU2C27uPHqc9S7fPfMqM7",
                true,
                "Valid XRPL address",
            ),
            (
                "PT1Sjq2YGrBMTU2C27uPHqc9S7fPfMqM7",
                false,
                "Missing leading 'r'",
            ),
            ("r123", false, "Too short"),
            ("rPT1Sjq2YGrBMTU2C27uPHqc9S7fPfMqM7extra", false, "Too long"),
        ];

        for (input, expected_ok, description) in test_cases {
            let result = Address::new(input.to_string());
            assert_eq!(
                result.is_ok(),
                expected_ok,
                "Failed test case: {}",
                description
            );
        }
    }
}
