use std::{fs, time::Instant};

use anyhow::Result;

use serde::Serialize;
use walkdir::WalkDir;

#[derive(Debug, Serialize, Clone, Copy)]
pub enum ScanProgress {
    Queued,
    LastCompleted(u128),
    Running,
}

pub type StatusMap = dashmap::DashMap<String, ScanProgress>;

#[derive(Default)]
pub struct GGater<D>
where
    D: TagBackend,
{
    status: StatusMap,
    search_directories: Vec<String>,
    plugins: Vec<String>,
    database: D,
}

impl<D> GGater<D>
where
    D: TagBackend,
{
    pub fn new(d: D) -> Self {
        Self {
            database: d,
            search_directories: Vec::new(),
            status: Default::default(),
            plugins: Default::default(),
        }
    }

    pub fn get_status(&self) -> &StatusMap {
        &self.status
    }

    pub fn scan(&self) -> Result<(), ()> {
        for plugin in self.plugins.iter() {
            self.status
                .insert("plugin".to_string(), ScanProgress::Queued);
        }
        // the current approach is to load each module and then iterator through all of the files.
        for plugin in self.plugins.iter() {
            self.status
                .insert("plugin".to_string(), ScanProgress::Running);
            for search_directory in self.search_directories.iter() {
                for entry in WalkDir::new(search_directory)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    let data = fs::read(entry.path()).unwrap();
                }
            }
            self.status.insert(
                "plugin".to_string(),
                ScanProgress::LastCompleted(Instant::now().elapsed().as_millis()),
            );
        }
        // Finish by sweeping the old entries
        self.sweep()?;
        self.status.insert(
            "sweep".into(),
            ScanProgress::LastCompleted(Instant::now().elapsed().as_millis()),
        );
        Ok(())
    }

    /// Remove stale entries for files that have been moved out of the seach directories
    pub fn sweep(&self) -> Result<(), ()> {
        Ok(())
    }
}

pub struct SqliteBackend {
    conn: sqlite::Connection,
}
impl TagBackend for SqliteBackend {
    fn set(&self, id: impl AsRef<str>) {
        todo!()
    }
    fn set_tags(&self, id: impl AsRef<str>, tags: &[Tag]) -> Result<()> {
        let query = "
    CREATE TABLE users (name TEXT, age INTEGER);
    INSERT INTO users VALUES ('Alice', 42);
    INSERT INTO users VALUES ('Bob', 69);
";
        let stmt = self.conn.prepare(query)?.bind(())?;
        self.conn.execute(stmt)?;
        Ok(())
    }
    fn get_tags<'a>(&self, id: impl AsRef<str>) -> Result<Vec<Tag<'a>>> {
        todo!()
    }
}

pub trait TagBackend {
    fn set(&self, id: impl AsRef<str>);
    fn set_tags<'a>(&self, id: impl AsRef<str>, tags: &'a [Tag<'a>]) -> Result<()>;
    fn get_tags<'a>(&self, id: impl AsRef<str>) -> Result<Vec<Tag<'a>>>;
}

struct Tag<'s> {
    key: &'s str,
    value: &'s str,
}

#[cfg(test)]
mod test {
    use crate::*;

    pub struct MockBackend;
    impl TagBackend for MockBackend {
        fn set(&self, id: impl AsRef<str>) {
            todo!()
        }
        fn set_tags(&self, id: impl AsRef<str>, tags: &[Tag]) -> Result<()> {
            todo!()
        }
        fn get_tags<'a>(&self, id: impl AsRef<str>) -> Result<Vec<Tag<'a>>> {
            todo!()
        }
    }

    #[test]
    fn init_ggater_client() {
        let ggater = GGater {
            database: MockBackend,
            search_directories: Vec::new(),
            status: Default::default(),
            plugins: Default::default(),
        };

        ggater.scan().unwrap()
    }
}
