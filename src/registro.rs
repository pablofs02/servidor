use std::fs::{File, OpenOptions};
use std::io::Write;

pub struct Registro {
    archivo: Option<File>
}

impl Registro {
    pub fn iniciar() -> Self {
        OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open("registro.ldf")
            .map_or_else(
                |_| {
                    eprintln!("Error al abrir el registro");
                    println!("No se escribirá en registro");
                    Self { archivo: None }
                },
                |archivo| Self {
                    archivo: Some(archivo)
                }
            )
    }

    // Hacer que en vez de texto sea "[fecha - ip] texto" o similar
    pub fn escribir(&mut self, texto: &str) {
        if let Some(archivo) = &mut self.archivo {
            if let Err(e) = writeln!(archivo, "{texto}") {
                eprintln!("Error al escribir en el registro: {e}");
            }
        }
    }
}
