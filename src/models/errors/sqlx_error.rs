use snafu::Snafu;

/// Wrapper around [sqlx::Error] with backtrace support
#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub struct SqlxError {
    source: sqlx::Error,

    #[snafu(implicit)]
    location: snafu::Location,

    // For non snafu sources
    #[cfg(feature = "backtrace")]
    backtrace: snafu::Backtrace,
}
