use std::io::Write;
use std::net::TcpStream;

pub fn solicitar(mut conexion: TcpStream) {
    let respuesta = String::from(
        "HTTP/1.1 200 OK\r\n\
        Allow: OPTIONS, GET, HEAD\r\n\
        \r\n",
    );
    conexion.write_all(respuesta.as_bytes()).unwrap();
    conexion.flush().unwrap();
}
