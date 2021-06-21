use std::fmt;

#[derive(Debug)]
pub enum Error {
    Reqwest,
    Conversion,
    InvalidDestination,
    ParseLibrary,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Reqwest => write!(f, "An error occured in reqwest"),
            Error::Conversion => write!(f, "A type conversion error occurred"),
            Error::InvalidDestination => {
                write!(f, "An element was placed in an invalid destination")
            }
            Error::ParseLibrary => write!(f, "An error occurred in quick_xml"),
        }
    }
}

impl From<chrono::format::ParseError> for Error {
    fn from(_: chrono::format::ParseError) -> Error {
        Error::Conversion
    }
}

impl From<url::ParseError> for Error {
    fn from(_: url::ParseError) -> Error {
        Error::Conversion
    }
}

impl From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Error {
        Error::Reqwest
    }
}
