use core::str::FromStr as _;
use std::path::Path;
use std::path::PathBuf;

use sqlx::sqlite::SqliteConnectOptions;
use tracing::debug;

use crate::client::conn_pool::PoolManager;
use crate::client::conn_pool::TSConnectionPool;

pub struct Library {
    pub path: PathBuf,
    pub db: TSConnectionPool,
}

impl Library {
    pub fn try_new(path: PathBuf) -> Result<Self, crate::Error> {
        let root = Self::get_library_root(&path)?.ok_or(crate::Error::LibraryNotFound)?;
        let path = root.join(".TagStudio/ts_library.sqlite");

        let string_lossy = path.to_string_lossy();
        debug!("Openning DB `{}`", string_lossy);
        let optconn = SqliteConnectOptions::from_str(&string_lossy)?;
        let pool = PoolManager::create_pool(optconn);

        Ok(Self {
            path: root,
            db: pool,
        })
    }

    /// For a given path, find the folder that contains the root `.TagStudio` folder **with** a `ts_library.sqlite` file.
    pub fn get_library_root<T>(path: T) -> Result<Option<PathBuf>, crate::Error>
    where
        T: AsRef<Path>,
    {
        let mut path = path.as_ref();

        while !Self::is_folder_library_root(path)? {
            match path.parent() {
                Some(new_path) => path = new_path,
                None => return Ok(None),
            }
        }

        Ok(Some(path.to_path_buf()))
    }

    pub fn is_folder_library_root<T>(path: T) -> Result<bool, std::io::Error>
    where
        T: AsRef<Path>,
    {
        let mut path = path.as_ref().to_path_buf();

        path.push(".TagStudio");
        path.push("ts_library.sqlite");

        path.try_exists()
    }

    /// Create a new library in memory.
    pub fn in_memory() -> Result<Self, crate::Error> {
        let optconn = SqliteConnectOptions::from_str(":memory:")?;
        let pool = PoolManager::create_pool(optconn);

        Ok(Self {
            path: "".into(),
            db: pool,
        })
    }
}
