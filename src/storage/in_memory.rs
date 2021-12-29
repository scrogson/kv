use std::collections::HashMap;

#[derive(Default)]
pub struct InMemory {
    data: HashMap<String, i64>,
    min: Option<i64>,
    max: Option<i64>,
    sum: i64,
}

impl InMemory {
    pub fn get(&self, key: impl AsRef<str>) -> Option<&i64> {
        self.data.get(key.as_ref())
    }

    pub fn put(&mut self, key: impl ToString, value: i64) -> Option<i64> {
        let result = self.data.insert(key.to_string(), value);
        self.calculate_aggregates();
        self.sum += value;

        result
    }

    pub fn del(&mut self, key: impl AsRef<str>) -> Option<i64> {
        match self.data.remove(key.as_ref()) {
            Some(v) => {
                self.calculate_aggregates();
                self.sum -= v;
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
        self.sum
    }

    fn calculate_aggregates(&mut self) {
        self.min = None;
        self.max = None;

        for value in self.data.values() {
            match self.max {
                Some(current) => {
                    if current < *value {
                        self.max = Some(*value);
                    }
                }
                None => self.max = Some(*value),
            }

            match self.min {
                Some(current) => {
                    if current > *value {
                        self.min = Some(*value);
                    }
                }
                None => self.min = Some(*value),
            }
        }
    }
}
