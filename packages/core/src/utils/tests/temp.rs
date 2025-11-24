#![allow(dead_code)]
use crate::prelude::APP_NAME;
use chrono::Local;
use std::env::temp_dir;
use std::fs::create_dir_all;
use std::path::PathBuf;
use std::thread::current as current_thread;

pub struct TempDirectory {
    path: PathBuf,
}

impl TempDirectory {
    #[must_use]
    pub fn new() -> TempDirectory {
        Self { path: temp_dir() }
    }

    #[must_use]
    pub fn with_name(mut self) -> TempDirectory {
        self.path.push(APP_NAME);
        self
    }

    #[must_use]
    pub fn with_datetime(mut self) -> TempDirectory {
        self.path.push(get_datetime());
        self
    }

    #[must_use]
    pub fn with_test_name(mut self) -> TempDirectory {
        self.path.push(get_test_name_underscored());
        self
    }

    #[must_use]
    pub fn get_path(self) -> PathBuf {
        self.path
    }

    #[allow(clippy::absolute_paths)]
    pub fn create(self) -> Result<PathBuf, std::io::Error> {
        create_dir_all(&self.path)?;
        Ok(self.path)
    }
}

impl Default for TempDirectory {
    fn default() -> Self {
        Self::new().with_name().with_test_name().with_datetime()
    }
}

fn get_datetime() -> String {
    Local::now().to_rfc3339().replace(':', "_").clone()
}

fn get_test_name_underscored() -> String {
    current_thread()
        .name()
        .expect("should be able to get test name")
        .replace(':', "_")
}
