use std::fmt;

#[derive(PartialEq, Debug)]
pub(crate) struct Error {
    details: String,
    file: String,
    line: u32,
}

#[macro_export]
macro_rules! error {
    ($details:tt) => {
        Error::new($details, file!(), line!())
    };
    ($details:item) => {
        Error::new($details, file!(), line!())
    };
}

impl Error {
    pub fn new(details: &str, file: &str, line: u32) -> Self {
        Self {
            details: details.to_string(),
            file : file.to_string(),
            line,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}
