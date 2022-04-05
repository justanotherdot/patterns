use thiserror::Error;

#[derive(Error, Debug)]
pub enum FormatError {
    #[error("Invalid header (expected {expected:?}, got {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("Missing attribute: {0}")]
    MissingAttribute(String),
}

#[derive(Error, Debug)]
pub enum IoError {
    #[error(transparent)]
    Io {
        #[from]
        source: FormatError,
    },
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

pub fn io_error() -> Result<(), IoError> {
    Err(FormatError::MissingAttribute("Foo".to_string()))?
}

fn main() {
    let choice = 2;
    match choice {
        1 => get_attribute().unwrap_or_else(|err| {
            eprintln!("[thiserror-example] {}", err);
        }),
        2 => get_other_attribute().unwrap_or_else(|err| {
            eprintln!("[thiserror-example] {}", err);
        }),
        3 => io_error().unwrap_or_else(|err| {
            eprintln!("[thiserror-example] {}", err);
        }),
        _ => (),
    }
}
