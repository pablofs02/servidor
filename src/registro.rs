use std::fs::{File, OpenOptions};
use std::io::Write;

pub struct Registro {
    archivo: Archivo
}

enum Archivo {
    Existe(File),
    Ninguno
}

impl Registro {
    pub fn iniciar() -> Self {
        OpenOptions::new().create(true).write(true).append(true).open("registro.pfs").map_or_else(
            |_| {
                eprintln!("Error al abrir el registro");
                println!("No se escribirá en registro");
                Self { archivo: Archivo::Ninguno }
            },
            |archivo| Self { archivo: Archivo::Existe(archivo) }
        )
    }

    // Hacer que en vez de texto sea "[fecha - ip] texto" o similar
    pub fn escribir(&mut self, texto: &str) {
        if let Archivo::Existe(archivo) = &mut self.archivo {
            if let Err(e) = writeln!(archivo, "{texto}") {
                eprintln!("Error al escribir en el registro: {e}");
            }
        }
    }
}
