use std::fs::{File, OpenOptions};
use std::io::Write;
use std::collections::HashMap;
use std::path::PathBuf;


struct Database {
    map: HashMap<String, String>,
    auto_commit: bool,
    file: File,
}

impl Database {
    pub fn new(path: PathBuf, auto_commit: Option<bool>) -> Self {
        Self {
            map: HashMap::new(),
            auto_commit: auto_commit.unwrap_or(true),
            file: OpenOptions::new()
                              .read(true)
                              .write(true)
                              .create(true)
                              .open(path)
                              .unwrap()
        }
    }

    pub fn put(&mut self, key: &str, value: &str) {
        self.map.insert(key.to_string(), value.to_string());
        self.commit();
    }

    pub fn get(&self, key: &str) -> String {
        self.map.get(key).unwrap().to_string()
    }

    pub fn commit(&mut self) {
        self.file.set_len(0).unwrap();

        for (key, value) in &self.map {
            self.file.write_all(format!("{} {}\n", key, value).as_bytes()).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let db = Database::new(PathBuf::from("test"), None);
        assert_eq!(db.auto_commit, true);
    }

    #[test]
    fn test_put_and_get() {
        let mut db = Database::new(PathBuf::from("test"), None);

        db.put("key", "value");
        let value = db.get("key");

        assert_eq!(value, "value".to_string());
    }

    #[test]
    fn test_commit() {
        let mut db = Database::new(PathBuf::from("test"), None);

        db.put("key", "value");
        db.commit();
    }
}