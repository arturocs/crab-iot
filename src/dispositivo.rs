use crate::error::Error;
use crate::plugin::Plugin;
struct Dispositivo {
    nombre: String,
    solo_lectura: bool,
    plugin: Plugin,
}

impl Dispositivo {
    fn obtener_estado() -> String {
        todo!()
    }
    fn cambiar_estado() -> Result<(), Error<String>> {
        todo!()
    }
}
