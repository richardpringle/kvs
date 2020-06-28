use avro_rs::{self, Reader, Schema, Writer};
use fehler::{throw, throws};
use serde_json;
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{self, Read, Seek, SeekFrom},
    path::PathBuf,
};
use thiserror;

// 1 MB
const COMPACTION_TRIGGER_SIZE: u64 = 1_000_000;
const DB_FILE_NAME: &str = "kvs.db";
const INDEX_FILE_NAME: &str = "kvs-index.json";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    IoError(io::Error),
    #[error("{0}")]
    Avro(String),
    #[error("not found")]
    NotFound,
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Self::IoError(io_error)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

// impl Serialize for Command

const SCHEMA: &str = r#"
    {
        "type": "record",
        "name": "triple",
        "fields": [
            { "name": "key", "type": "string" },
            { "name": "value", "type": "string" }
        ]
    }
"#;

mod command;

use command::Command;

/// Currently, there's no real reason to use `KvStore` over using a regular `HashMap`
pub struct KvStore {
    index: HashMap<String, (u64, usize)>,
    index_path: PathBuf,
    storage_dir: PathBuf,
    file: File,
    schema: Schema,
}

impl KvStore {
    #[throws]
    pub fn open(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let mut index_path = path.clone();
        let mut file_path = path.clone();
        index_path.push(INDEX_FILE_NAME);
        file_path.push(DB_FILE_NAME);

        let schema = Schema::parse_str(SCHEMA).unwrap();
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(file_path)?;

        let index_file = fs::read(index_path.clone()).unwrap_or(b"{}".to_vec());

        let index: HashMap<String, (u64, usize)> = serde_json::from_slice(&index_file).unwrap();

        Self {
            index,
            index_path,
            storage_dir: path,
            file,
            schema,
        }
    }

    #[throws]
    pub fn set(&mut self, key: String, value: String) {
        let start = self.file.metadata()?.len();

        let mut writer = Writer::new(&self.schema, &self.file);
        let mut len = 0;

        len += writer
            .append_ser(Command(key.clone(), value))
            .map_err(|err| Error::Avro(String::from(err.name().unwrap_or("unknown"))))?;
        len += writer
            .flush()
            .map_err(|err| Error::Avro(String::from(err.name().unwrap_or("unknown"))))?;

        self.index.insert(key, (start, len));

        if start > COMPACTION_TRIGGER_SIZE {
            self.compact()?;
        }
    }

    #[throws]
    pub fn get(&mut self, key: String) -> Option<String> {
        if !self.index.contains_key(&key) {
            return None;
        }

        let (start, len) = *self.index.get(&key).unwrap();

        let mut bytes = vec![0u8; len];
        self.file.seek(SeekFrom::Start(start)).unwrap();
        self.file.read_exact(&mut bytes).unwrap();

        let mut reader = Reader::with_schema(&self.schema, bytes.as_slice())
            .map_err(|err| Error::Avro(String::from(err.name().unwrap_or("unknown"))))?;

        let value = reader
            .next()
            .map(|value| value.unwrap())
            .map(|value| avro_rs::from_value::<Command>(&value).unwrap())
            .unwrap()
            .1;

        Some(value)
    }

    #[throws]
    pub fn remove(&mut self, key: String) {
        if !self.index.contains_key(&key) {
            throw!(Error::NotFound);
        }

        self.index.remove(&key);
    }

    #[throws]
    fn compact(&mut self) {
        let mut temp_dir = self.storage_dir.clone();
        temp_dir.push("tmp/");

        let new_index = {
            fs::create_dir(temp_dir.clone())?;
            let mut new_store = Self::open(temp_dir.clone())?;

            let keys: Vec<String> = self.index.keys().map(|key| key.clone()).collect();

            for key in keys {
                let value = self.get(key.clone())?.unwrap();
                new_store.set(key, value)?;
            }

            std::mem::take(&mut new_store.index)
        };

        let mut compacted_log_path = temp_dir.clone();
        compacted_log_path.push(DB_FILE_NAME);

        let mut current_log_path = self.storage_dir.clone();
        current_log_path.push(DB_FILE_NAME);

        fs::copy(compacted_log_path, current_log_path)?;

        self.index = new_index;

        fs::remove_dir_all(temp_dir)?;
    }
}

impl Drop for KvStore {
    fn drop(&mut self) {
        let bytes = serde_json::to_vec(&self.index).unwrap();
        fs::write(self.index_path.clone(), bytes).unwrap();
    }
}
