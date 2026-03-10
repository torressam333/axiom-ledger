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
