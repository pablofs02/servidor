use super::{dar_respuesta, error};
use crate::Opciones;
use std::fs;
use std::net::TcpStream;

pub fn solicitar(conexion: TcpStream, mut archivo: String, estatus: &str, opciones: Opciones) -> Result<(), usize> {
    let mut error301 = false;
    if archivo == "/" {
        archivo.push_str("index.html");
    } else {
        let _ = fs::metadata(&archivo[1..]).ok().map_or((), |metadata| {
            if metadata.is_dir() {
                if archivo.ends_with('/') {
                    archivo.push_str("/index.html");
                } else {
                    archivo.push('/');
                    error301 = true;
                }
            }
        });
    }
    if error301 {
        error::movido_301(conexion, &archivo);
    } else {
        match fs::read(&archivo[1..]) {
            Ok(contenido) => dar_respuesta(conexion, estatus, &archivo, &contenido),
            Err(_) => error::no_encontrado_404(conexion, &archivo, opciones),
        }
    }
    Ok(())
}
