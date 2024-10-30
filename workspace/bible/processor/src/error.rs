use quick_xml::events::attributes::AttrError;
use std::fmt::{Display, Formatter};
use std::str::Utf8Error;
use zip::result::ZipError;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl Error {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self {
            message: format!("{}", value),
        }
    }
}

impl From<quick_xml::Error> for Error {
    fn from(value: quick_xml::Error) -> Self {
        Self {
            message: format!("{}", value),
        }
    }
}

impl From<Utf8Error> for Error {
    fn from(value: Utf8Error) -> Self {
        Self {
            message: format!("{}", value),
        }
    }
}

impl From<AttrError> for Error {
    fn from(value: AttrError) -> Self {
        Self {
            message: format!("{}", value),
        }
    }
}

impl From<quick_xml::DeError> for Error {
    fn from(value: quick_xml::DeError) -> Self {
        Self {
            message: format!("{}", value),
        }
    }
}

impl From<ZipError> for Error {
    fn from(value: ZipError) -> Self {
        Self {
            message: format!("{}", value),
        }
    }
}
