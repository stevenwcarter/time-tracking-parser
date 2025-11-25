use super::*;

/// Represents aggregated project data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProjectSummary {
    pub name: String,
    pub total_minutes: u32,
    pub notes: Vec<String>,
}

impl ProjectSummary {
    pub fn new(name: String) -> Self {
        ProjectSummary {
            name,
            total_minutes: 0,
            notes: Vec::new(),
        }
    }

    pub fn add_time(&mut self, minutes: u32) {
        self.total_minutes += minutes;
    }

    pub fn add_notes(&mut self, notes: Vec<String>) {
        self.notes.extend(notes);
    }
}
