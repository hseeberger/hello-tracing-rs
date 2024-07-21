pub mod config;
pub mod otel;
pub mod tracing;

use serde_json::json;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

pub fn log_error(error: &anyhow::Error) {
    let now = OffsetDateTime::now_utc().format(&Rfc3339).unwrap();
    let error = serde_json::to_string(&json!({
        "timestamp": now,
        "level": "ERROR",
        "message": "process exited with ERROR",
        "error": format!("{error:#}")
    }));
    // Not using `eprintln!`, because `tracing_subscriber::fmt` uses stdout by default.
    println!("{}", error.unwrap());
}
