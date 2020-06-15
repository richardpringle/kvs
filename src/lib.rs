pub struct KvStore;

impl KvStore {
    pub fn new() -> Self {
        Self
    }

    pub fn get<T>(&self, _key: T) -> Option<T> {
        todo!()
    }

    pub fn set<T>(&mut self, _key: T, _value: T) {
        todo!()
    }

    pub fn remove<T>(&mut self, _key: T) {
        todo!()
    }
}
