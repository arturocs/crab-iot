use std::fmt;

#[derive(PartialEq, Debug)]
pub(crate) struct Error {
    detalles: String,
    archivo: String,
    linea: u32,
}

macro_rules! error {
    ($descripcion:tt) => {
        Error::new($descripcion, file!(), line!())
    };
    ($descripcion:item) => {
        Error::new($descripcion, file!(), line!())
    };
}

impl Error {
    pub fn new(detalles: &str, archivo: &str, linea: u32) -> Self {
        Self {
            detalles: detalles.to_string(),
            archivo : archivo.to_string(),
            linea,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.detalles)
    }
}
