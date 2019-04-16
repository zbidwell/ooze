use std::error;
use std::fmt;

pub type OozeResult<T> = std::result::Result<T, Box<error::Error>>;

/// Ooze's custom generic error.
#[derive(Debug)]
pub struct OozeError;
// TODO: Add a string to track the error's cause?

impl fmt::Display for OozeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An error occured while oozing")
    }
}

impl error::Error for OozeError {
    fn description(&self) -> &str {
        "An error occured while oozing"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}