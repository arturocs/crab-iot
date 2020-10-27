use crate::error::Error;
use std::path::PathBuf;

pub(crate) struct Plugin {
    nombre_dispositivo: String,
    id_dispositivo: String,
    ruta_plugin: PathBuf,
}

impl Plugin {
    fn cargar(nombre: &str, ruta: &str) -> Plugin {
        todo!()
    }
    fn ejecutar(&self) -> Result<(), Error<String>> {
        todo!()
    }
    fn parar_ejecucion(&self) -> Result<(), Error<String>> {
        todo!()
    }
}
