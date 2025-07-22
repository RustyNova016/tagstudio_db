use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Couldn't find the library")]
    LibraryNotFound,

    #[error("The path given isn't part of the folder")]
    PathNotInFolder,

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    IO(#[from] io::Error),
}
