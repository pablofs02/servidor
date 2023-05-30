mod error;
mod get;
use super::Opciones;
use std::fs::OpenOptions;
use std::io::prelude::Write;
use std::net::TcpStream;
use urlencoding::decode;

pub fn tratar(conexion: TcpStream, solicitud: &str, opciones: Opciones) {
    let (tipo, archivo, mut estatus) = desmontar_solicitud(solicitud);
    let archivo = decode(&archivo).expect("UTF-8");
    if opciones.verboso {
        conexion.peer_addr().map_or_else(|_| println!("{tipo} {archivo} {estatus}"), |dir| println!("[{}] {tipo} {archivo} {estatus}", dir.ip()));
    }
    let ip = conexion.peer_addr().unwrap().ip();
    let mut registro = OpenOptions::new().write(true).append(true).open("registro.pfs").unwrap();
    if let Err(e) = writeln!(registro, "[{ip}] {tipo} {archivo} {estatus}") {
        eprintln!("Error al registrar: {e}");
    }
    estatus.push_str(" 200 OK");
    match &tipo[..] {
        "GET" => get::solicitar(conexion, archivo.to_string(), &estatus),
        _ => solicitud_desconocida(conexion)
    }
}

fn solicitud_desconocida(mut conexion: TcpStream) {
    let respuesta = "HTTP/1.1 501 Not Implemented";
    conexion.write_all(respuesta.as_bytes()).unwrap();
    conexion.flush().unwrap();
}

fn dar_respuesta(mut conexion: TcpStream, estatus: &str, archivo: &str, contenido: &[u8]) {
    let longitud = contenido.len();
    let tipo = sacar_tipo(archivo).to_string();
    let respuesta = format!("{estatus}\r\nContent-Type: {tipo}\r\nContent-Length: {longitud}\r\n\r\n");
    conexion.write_all(respuesta.as_bytes()).unwrap();
    conexion.write_all(contenido).unwrap();
    conexion.flush().unwrap();
}

fn desmontar_solicitud(solicitud: &str) -> (String, String, String) {
    let mut estado = 0;
    let mut tipo = String::new();
    let mut contenido = String::new();
    let mut version = String::new();
    for c in solicitud.chars() {
        if estado == 0 {
            if c == ' ' {
                estado += 1;
            } else {
                tipo.push(c);
            }
        } else if estado == 1 {
            if c == ' ' {
                estado += 1;
            } else {
                contenido.push(c);
            }
        } else {
            version.push(c);
        }
    }
    (tipo, contenido, version)
}

fn sacar_tipo(archivo: &str) -> &str {
    let tipo = sacar_extension(archivo);
    match &tipo[..] {
        "css" => "text/css",
        "gif" => "image/gif",
        "html" => "text/html",
        "jpg" | "jpeg" => "image/jpeg",
        "js" => "application/javascript",
        "json" => "application/json",
        "mp3" | "mpeg" => "audio/mpeg",
        "mp4" => "video/mp4",
        "pdf" => "application/fdf",
        "png" => "image/png",
        "svg" => "image/svg+xml; charset=utf-8",
        "obj" => "model/obj",
        "ogg" | "oga" => "audio/ogg",
        "ogv" => "video/ogg",
        "otf" => "font/otf",
        "ttf" => "font/ttf",
        "weba" | "webm" => "audio/webm",
        "webp" => "image/webp",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "zip" => "application/zip",
        _ => ""
    }
}

fn sacar_extension(archivo: &str) -> String {
    let mut tipo = String::new();
    for c in archivo.chars() {
        if c == '.' {
            tipo = String::new();
        } else {
            tipo.push(c);
        }
    }
    tipo
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn buena_extension() {
        let archivo = "datos.csv";
        let esperado = "csv".to_string();
        let obtenido = sacar_extension(archivo);
        assert_eq!(esperado, obtenido);
    }

    #[test]
    fn buen_tipo() {
        let archivo = "funciones.js";
        let esperado = "application/javascript".to_string();
        let obtenido = sacar_tipo(archivo);
        assert_eq!(esperado, obtenido);
    }
}
