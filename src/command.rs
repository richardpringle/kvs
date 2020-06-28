use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug)]
pub struct OwnedCommand(pub String, pub Option<String>);

#[derive(Serialize, Deserialize)]
struct OwnedKeyedCommand {
    key: String,
    value: Option<String>,
}

pub struct BorrowedCommand<'a>(pub &'a str, pub Option<&'a str>);

#[derive(Serialize, Deserialize)]
struct BorrowedKeyedCommand<'a> {
    key: &'a str,
    value: Option<&'a str>,
}

impl<'a> BorrowedKeyedCommand<'a> {
    #[inline]
    fn new(key: &'a str, value: Option<&'a str>) -> Self {
        Self { key, value }
    }
}

impl<'a> Serialize for BorrowedCommand<'a> {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let Self(key, value) = self;
        BorrowedKeyedCommand::new(*key, *value).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for OwnedCommand {
    #[inline]
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let OwnedKeyedCommand { key, value } = OwnedKeyedCommand::deserialize(deserializer)?;
        Ok(OwnedCommand(key, value))
    }
}
