use std::error;
use std::fmt;

pub type OozeResult<T> = std::result::Result<T, Box<error::Error>>;

#[derive(Debug)]
pub struct OozeError;

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

// impl From<ProgramCreationError> for OozeError {
//     fn from(err: ProgramCreationError) -> OozeError {
//         OozeError
//     }
// }

// impl From<DisplayCreationError> for OozeError {
//     fn from(err: DisplayCreationError) -> OozeError {
//         OozeError
//     }
// }

// impl From<io::Error> for OozeError {
//     fn from(err: io::Error) -> OozeError {
//         OozeError
//     }
// }

// impl<E: error::Error> From<E> for OozeError {
//     fn from(err: E) -> OozeError {
//         OozeError
//     }
// }