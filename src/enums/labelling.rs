#[derive(Debug, Clone)]
pub enum Labelling {
    Numeric,
    Alphabetic,
    AlphaNumeric
}


impl From<&str> for Labelling {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "alphabetic" => Self::Alphabetic,
            "alphanumeric" => Self::AlphaNumeric,
            _ => Self::Numeric,
        }
    }
}

impl Into<&str> for Labelling {
    fn into(self) -> &'static str {
        match self {
            Self::AlphaNumeric => "alphanumeric",
            Self::Alphabetic => "alphabetic",
            Self::Numeric => "numeric",
        }
    }
}
