use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug)]
pub struct Command(pub String, pub String);

#[derive(Serialize, Deserialize)]
struct OwnedKeyedCommand {
    key: String,
    value: String,
}

#[derive(Serialize, Deserialize)]
struct BorrowedKeyedCommand<'a> {
    key: &'a str,
    value: &'a str,
}

impl<'a> BorrowedKeyedCommand<'a> {
    #[inline]
    fn new(key: &'a str, value: &'a str) -> Self {
        Self { key, value }
    }
}

impl Serialize for Command {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let Self(key, value) = self;
        BorrowedKeyedCommand::new(key, value).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Command {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let OwnedKeyedCommand { key, value } = OwnedKeyedCommand::deserialize(deserializer)?;
        Ok(Command(key, value))
    }
}
