use anyhow::{anyhow, Context, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FormatError {
    #[error("Invalid header (expected {expected:?}, got {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("Missing attribute: {0}")]
    MissingAttribute(String),
}

pub fn get_attribute() -> Result<(), FormatError> {
    Err(FormatError::InvalidHeader {
        expected: "foo".to_string(),
        found: "bar".to_string(),
    })
}

pub fn get_other_attribute() -> Result<(), FormatError> {
    Err(FormatError::MissingAttribute("Foo".to_string()))
}

pub fn get_cluster_info() -> Result<()> {
    std::fs::read_to_string("cluster.json")?;
    Ok(())
}

// RUST_BACKTRACE=1 will pretty print a backtrace.
fn main() -> Result<()> {
    let choice = 4;
    match choice {
        1 => get_cluster_info().context("could not find cluster.json"),
        2 => get_attribute().map_err(Into::into),
        3 => get_other_attribute().map_err(Into::into),
        4 => Err(anyhow!("[anyhow-example]: {}", "urk")),
        _ => Ok(()),
    }
}
