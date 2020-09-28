#[derive(Debug, Default, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Name(pub String);

impl From<String> for Name {
    fn from(string: String) -> Self {
        Name(string)
    }
}

impl Into<String> for Name {
    fn into(self) -> String {
        self.0
    }
}
