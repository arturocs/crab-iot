use crate::error::Error;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Plugin {
    nombre_dispositivo: String,
    id_dispositivo: String,
    ruta_plugin: PathBuf,
}

impl Plugin {
    fn cargar(nombre: &str, ruta: &str) -> Plugin {
        todo!()
    }
    fn ejecutar(&self) -> Result<(), Error> {
        todo!()
    }
    fn parar_ejecucion(&self) -> Result<(), Error> {
        todo!()
    }
}
