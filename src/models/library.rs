use core::str::FromStr as _;
use core::time::Duration;
use std::backtrace::Backtrace;
use std::path::Path;
use std::path::PathBuf;

use snafu::OptionExt;
use snafu::ResultExt;
use snafu::Snafu;
use sqlx::sqlite::SqliteConnectOptions;
use tracing::debug;

use crate::TSPoolError;
use crate::client::conn_pool::PoolManager;
use crate::client::conn_pool::TSConnectionPool;
use crate::models::folder::Folder;

/// A struct representing a TagStudio library.
pub struct Library {
    pub path: PathBuf,
    pub db: TSConnectionPool,
}

impl Library {
    /// Try to open the library. Returns the same errors than [Library::open_library], but also check if the database version is properly matched.
    pub async fn try_open_library(path: PathBuf) -> Result<Self, LibraryTryOpenError> {
        let lib = Self::open_library(path).context(OpenSnafu)?;

        let res = sqlx::query_scalar!("SELECT VALUE FROM `versions` WHERE `key` = 'CURRENT'")
            .fetch_one(&mut *lib.db.get().await.context(SqlPoolSnafu)?)
            .await;

        let version = res.unwrap_or(100);

        if version >= 101 && version % 100 == 1 {
            Ok(lib)
        } else {
            Err(LibraryTryOpenError::IncompatibleVersion {
                lib_version: version,
                allowed_version: 101,
                backtrace: Backtrace::capture(),
            })
        }
    }

    /// Opens the library that contains the given path. Fails if the DB cannot be found, or cannot be opened
    pub fn open_library(path: PathBuf) -> Result<Self, LibraryOpenError> {
        let root =
            Self::get_library_root(&path)
                .context(IoSnafu)?
                .context(LibraryNotFoundSnafu {
                    path: path.display().to_string(),
                })?;
        let path = root.join(".TagStudio/ts_library.sqlite");

        let string_lossy = path.to_string_lossy();
        debug!("Openning DB `{}`", string_lossy);
        let optconn = SqliteConnectOptions::from_str(&string_lossy)
            .context(SqlSnafu)?
            .busy_timeout(Duration::from_secs(600));
        let pool = PoolManager::create_pool(optconn);

        Ok(Self {
            path: root,
            db: pool,
        })
    }

    pub fn try_new(path: PathBuf) -> Result<Self, crate::Error> {
        let root = Self::get_library_root(&path)?.ok_or(crate::Error::LibraryNotFound)?;
        let path = root.join(".TagStudio/ts_library.sqlite");

        let string_lossy = path.to_string_lossy();
        debug!("Openning DB `{}`", string_lossy);
        let optconn =
            SqliteConnectOptions::from_str(&string_lossy)?.busy_timeout(Duration::from_secs(600));
        let pool = PoolManager::create_pool(optconn);

        Ok(Self {
            path: root,
            db: pool,
        })
    }

    /// Find the library root from a path within the library.
    ///
    /// If the path isn't in a library, returns [`None`]
    pub fn get_library_root<T>(path: T) -> Result<Option<PathBuf>, std::io::Error>
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

    /// Checks if a path is the root of a TagStudio library
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

    /// Get the [`Folder`] that is at the root of the directory
    pub async fn get_root_db_folder(
        &self,
        conn: &mut sqlx::SqliteConnection,
    ) -> Result<Folder, sqlx::Error> {
        let path = self.path.to_string_lossy();
        sqlx::query_as!(Folder, "SELECT * FROM `folders` WHERE path = $1", path)
            .fetch_one(conn)
            .await
    }
}

/// Error for [Library::open_library]
#[derive(Debug, Snafu)]
pub enum LibraryOpenError {
    #[snafu(display("Filesytem returned an error"))]
    IoError {
        source: std::io::Error,
        backtrace: Backtrace,
    },

    #[snafu(display("Couldn't find a library in: {path}"))]
    LibraryNotFound { path: String, backtrace: Backtrace },

    #[snafu(display("Sqlite returned an error"))]
    SqlError {
        source: sqlx::Error,
        backtrace: Backtrace,
    },
}

/// Error for [Library::try_open_library]
#[derive(Debug, Snafu)]
pub enum LibraryTryOpenError {
    #[snafu(display("Error while opening the library"))]
    OpenError {
        #[snafu(backtrace)]
        source: LibraryOpenError,
    },

    #[snafu(display(
        "The current library version in incompatible with the version of the crate ({lib_version} vs {allowed_version}). Please upgrade the library to version {allowed_version}, or update the crate to version {lib_version}"
    ))]
    IncompatibleVersion {
        lib_version: i64,
        allowed_version: i64,
        backtrace: Backtrace,
    },

    #[snafu(display("Sqlite returned an error"))]
    SqlPoolError {
        source: TSPoolError,
        backtrace: Backtrace,
    },
}
