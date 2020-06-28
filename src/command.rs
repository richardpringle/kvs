use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug)]
pub struct Command(pub String, pub Option<String>);

#[derive(Serialize, Deserialize, Clone)]
struct KeyedCommand {
    key: String,
    value: Option<String>,
}

impl Command {
    #[inline]
    fn as_keyed_command(&self) -> KeyedCommand {
        let Self(key, value) = self.clone();
        KeyedCommand { key, value }
    }
}

impl Serialize for Command {
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_keyed_command().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Command {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let KeyedCommand { key, value } = KeyedCommand::deserialize(deserializer)?;
        Ok(Command(key, value))
    }
}
