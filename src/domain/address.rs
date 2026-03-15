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
