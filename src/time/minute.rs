use super::*;

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(transparent)]
pub struct Minute(u8);

impl Minute {
    pub fn get(&self) -> u8 {
        self.0
    }
}

impl PartialEq<u8> for Minute {
    fn eq(&self, other: &u8) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<u8> for Minute {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl Display for Minute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<u8> for Minute {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(0..=60).contains(&value) {
            return Err(format!("Minute must be between 0 and 60, got {}", value));
        }
        Ok(Minute(value))
    }
}

impl FromStr for Minute {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hour: u8 = s
            .parse()
            .map_err(|_| format!("Invalid hour format: {}", s))?;
        if !(0..=60).contains(&hour) {
            return Err(format!("Minute must be between 0 and 60, got {}", hour));
        }
        Ok(Minute(hour))
    }
}
