use std::path::PathBuf;
use crate::error::Error;

pub(crate) struct Plugin {
    nombre_dispositivo: String,
    id_dispositivo: String,
    ruta_plugin: PathBuf,
}

impl Plugin {
    fn cargar(ruta: &str) -> Plugin {
        todo!()
    }
    fn ejecutar() -> Result<(), Error<String>> {
        todo!()
    }
}
