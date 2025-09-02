use std::io;
use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Couldn't find the library")]
    LibraryNotFound,

    #[error("Tried to move the entry to a path outside of the entry's folder")]
    PathNotInFolder,

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    IO(#[from] io::Error),

    #[error("Source file {0} is missing on disk")]
    MissingSourceFile(String),

    #[error("Couldn't move file due to another file already being present at `{0}`")]
    DestinationOccupied(PathBuf),
}
