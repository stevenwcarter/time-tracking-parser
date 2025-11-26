use super::*;

use nutype::nutype;

#[nutype(
    derive(
        Debug,
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
            (0..=59).contains(value)
        }
    )
)]
pub struct Minute(u8);

impl Minute {
    pub fn get(&self) -> u8 {
        *self.as_ref()
    }
}

impl Display for Minute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.as_ref())
    }
}

impl PartialEq<u8> for Minute {
    fn eq(&self, other: &u8) -> bool {
        self.as_ref() == other
    }
}

impl PartialOrd<u8> for Minute {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.as_ref().partial_cmp(other)
    }
}

impl TryFrom<u8> for Minute {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Minute::try_new(value).map_err(|e| e.to_string())
    }
}

impl FromStr for Minute {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hour: u8 = s
            .parse()
            .map_err(|_| format!("Invalid hour format: {}", s))?;
        Minute::try_new(hour).map_err(|e| e.to_string())
    }
}
