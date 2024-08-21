use serde_json::json;
use std::{error::Error as StdError, fmt::Display};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

/// Alias for `async` and `anyhow` friendly dynamic error
/// `Box<dyn std::error::Error + Send + Sync + 'static>`.
#[allow(unused)]
pub type BoxError = Box<dyn StdError + Send + Sync + 'static>;

/// Extension methods for types implementing `std::error::Error`.
pub trait StdErrorExt
where
    Self: StdError,
{
    /// Format this error as a chain of colon separated strings built from this error and all
    /// recursive sources.
    ///
    /// Can be used to log errors like this:
    ///
    /// `error!(error = error.as_chain(), "cannot do this or that");`
    fn as_chain(&self) -> String {
        let mut sources = vec![];
        sources.push(self.to_string());

        let mut source = self.source();
        while let Some(s) = source {
            sources.push(s.to_string());
            source = s.source();
        }

        sources.join(": ")
    }
}

impl<T> StdErrorExt for T where T: StdError {}

/// Log an error before structured logging, e.g. via Tokio Tracing, has been initialized in a
/// similar structured way.
pub fn log_error<T>(error: &T)
where
    T: Display + ?Sized,
{
    let now = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .expect("now can be Rfc3339 formatted");

    let error = serde_json::to_string(&json!({
        "timestamp": now,
        "level": "ERROR",
        "message": "process exited with ERROR",
        "error": format!("{error:#}")
    }));

    // Not using `eprintln!`, because `tracing_subscriber::fmt` (Tokio Tracing) uses stdout.
    println!("{}", error.unwrap());
}

#[cfg(test)]
mod tests {
    use crate::error::StdErrorExt;
    use std::num::ParseIntError;
    use thiserror::Error;

    #[test]
    fn test_as_chain() {
        let number = "-1".parse::<u32>().map_err(Error);
        assert_eq!(
            number.unwrap_err().as_chain(),
            "error: invalid digit found in string"
        );
    }

    #[derive(Debug, Error)]
    #[error("error")]
    struct Error(#[source] ParseIntError);
}
