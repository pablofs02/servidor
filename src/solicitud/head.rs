use super::{error, tipo};
use crate::Opciones;
use std::fs;
use std::io::Write;
use std::net::TcpStream;

pub fn solicitar(mut conexion: TcpStream, mut archivo: String, estatus: &str, opciones: Opciones) {
    // comprobar si las rutas comienzan por '/'.
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
            Ok(contenido) => {
                let longitud = contenido.len();
                let tipo = tipo::sacar(&archivo).to_string();
                let respuesta = format!(
                    "{estatus}\r\nContent-Type: {tipo}\r\nContent-Length: {longitud}\r\n\r\n"
                );
                conexion.write_all(respuesta.as_bytes()).unwrap();
                conexion.flush().unwrap();
            }
            Err(_) => error::no_encontrado_404(conexion, &archivo, opciones),
        }
    }
}
