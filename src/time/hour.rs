use super::*;

use nutype::nutype;

#[nutype(
    derive(
        Debug,
        Display,
        Copy,
        Clone,
        Deserialize,
        Serialize,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        AsRef,
        Borrow
    ),
    validate(predicate =
        |value: &u8| {
            (0..=12).contains(value)
        }
    )
)]
pub struct Hour(u8);

impl Hour {
    pub fn get(&self) -> u8 {
        *self.as_ref()
    }
}

impl PartialEq<u8> for Hour {
    fn eq(&self, other: &u8) -> bool {
        *self.as_ref() == *other
    }
}

impl PartialOrd<u8> for Hour {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.as_ref().partial_cmp(other)
    }
}

impl TryFrom<u8> for Hour {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Hour::try_new(value).map_err(|e| e.to_string())
    }
}

impl FromStr for Hour {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hour: u8 = s
            .parse()
            .map_err(|_| format!("Invalid hour format: {}", s))?;
        Hour::try_new(hour).map_err(|e| e.to_string())
    }
}
