use std::fs::{File, OpenOptions};
use std::io::Write;

pub struct Registro {
    archivo: File
}

impl Registro {
    pub fn iniciar() -> Self {
        let archivo = OpenOptions::new().create(true).write(true).append(true).open("registro.pfs").unwrap();
        Self { archivo }
    }

    pub fn escribir(&mut self, texto: &str) {
        if let Err(e) = writeln!(self.archivo, "{texto}") {
            eprintln!("No se pudo escribir en el registro: {e}");
        }
    }
}
