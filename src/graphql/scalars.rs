use crate::domain::balance::Balance;
use async_graphql::{Scalar, ScalarType, Value};
use std::str::FromStr;

#[Scalar]
impl ScalarType for Balance {
    // Function to parse incoming gql json value into my Rust Balance
    fn parse(value: Value) -> async_graphql::InputValueResult<Self> {
        match value {
            // High precision financial data should be a string...floats and ints aren't reliable here
            value::String(s) => {
                let parsed = u128::from_str(&s).map_err(|_| {
                    async_graphqlInputValueError::custom("Invalid u128 format for Balance")
                })?;

                Ok(Balance::new(parsed))
            }
            _ => Err(async_graphql::InputValueError::expected_type(value)),
        }
    }

    // Function to parse balance back into JSON
    fn to_value(&self) -> Value {
        // Value fn comes from balance.rs for Balance impl
        Value::String(self.value().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_graphql::{ScalarType, Value};

    #[test]
    fn test_scalar_to_balance_value() {
        let balance = Balance::new(100);
        let value = ScalarType::to_value(&balance);

        // Should have string value to preserve precisioon
        assert_eq!(value, Value::String("100".to_string()))
    }

    #[test]
    fn test_scalar_balance_parse_success() {
        let raw_value = Value::String("100".to_string());
        let parsed = Balance::parse(raw_value).expect("Should parse balance successfully");

        assert_eq!(parsed.value(), 500)
    }

    #[test]
    fn test_balance_scalar_parse_failure() {
        let fake_string = Value::String("not_a_number".to_string());
        assert!(Balance::parse(fake_string).is_err());

        // Test 2: Wrong type (Number instead of String)
        // Reject numbers because JS might have already truncated them!
        let wrong_type = Value::Number(500.into());
        assert!(Balance::parse(wrong_type).is_err());
    }
}
