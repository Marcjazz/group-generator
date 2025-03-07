use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Difficulty {
    Hard,
    Medium,
    Easy,
}

impl From<&str> for Difficulty {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "hard" => Self::Hard,
            "medium" => Self::Medium,
            _ => Self::Easy,
        }
    }
}

impl Into<&str> for Difficulty {
    fn into(self) -> &'static str {
        match self {
            Self::Hard => "Hard",
            Self::Medium => "Medium",
            Self::Easy => "Easy",
        }
    }
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
