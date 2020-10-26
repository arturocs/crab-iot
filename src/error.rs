use std::fmt::{self, Debug};

#[derive(Debug, PartialEq)]
pub(crate) struct Error<T>
where
    T: Debug,
{
    details: T,
}

impl<T: Debug> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
