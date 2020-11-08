use std::fmt;

#[derive(PartialEq, Debug)]
pub(crate) struct Error {
    details: String,
    file: String,
    line: u32,
}

#[macro_export]
macro_rules! error {
    //string literals
    ($details:tt) => {
        Error::new($details, file!(), line!())
    };
    //&str
    ($details:expr) => {
        Error::new($details, file!(), line!())
    };
}

impl std::error::Error for Error {}

impl Error {
    pub(crate) fn new(details: impl fmt::Display, file: &str, line: u32) -> Self {
        Self {
            details: details.to_string(),
            file: file.to_string(),
            line,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self.details)
    }
}
