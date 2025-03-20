use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Labelling {
    Numeric,
    Alphabetic,
    Alphanumeric
}


impl From<&str> for Labelling {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "alphabetic" => Self::Alphabetic,
            "alphanumeric" => Self::Alphanumeric,
            _ => Self::Numeric,
        }
    }
}

impl Into<&str> for Labelling {
    fn into(self) -> &'static str {
        match self {
            Self::Alphanumeric => "alphanumeric",
            Self::Alphabetic => "alphabetic",
            Self::Numeric => "numeric",
        }
    }
}
