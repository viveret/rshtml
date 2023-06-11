use std::{fmt, borrow::Cow};

pub struct ActionPath(pub String);


impl ActionPath {
    pub fn is_equivalent_to(self: &Self, other: &str) -> bool {
        self.as_str() == other ||
        self.ends_with(other) ||
        other.ends_with(self.as_str())
    }

    pub fn as_str(self: &Self) -> &str {
        self.0.as_str()
    }

    pub fn to_cow_str(self: &Self) -> Cow<'static, str> {
        Cow::Owned(self.0.clone())
    }

    pub fn ends_with(self: &Self, other: &str) -> bool {
        self.0.ends_with(other)
    }
}

impl fmt::Display for ActionPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
