use std::collections::HashMap;

#[derive(Default)]
pub struct InMemory {
    data: HashMap<String, i64>,
    min: Option<i64>,
    max: Option<i64>,
}

impl InMemory {
    pub fn get(&self, key: impl AsRef<str>) -> Option<&i64> {
        self.data.get(key.as_ref())
    }

    pub fn put(&mut self, key: impl ToString, value: i64) -> Option<i64> {
        let result = self.data.insert(key.to_string(), value);
        self.min = self.data.values().min().copied();
        self.max = self.data.values().max().copied();

        result
    }

    pub fn del(&mut self, key: impl AsRef<str>) -> Option<i64> {
        match self.data.remove(key.as_ref()) {
            Some(v) => {
                self.min = self.data.values().min().copied();
                self.max = self.data.values().max().copied();
                Some(v)
            }
            None => None,
        }
    }

    pub fn min(&self) -> Option<&i64> {
        self.min.as_ref()
    }

    pub fn max(&self) -> Option<&i64> {
        self.max.as_ref()
    }

    pub fn sum(&self) -> i64 {
        self.data.values().sum()
    }
}
