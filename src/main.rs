use servidor::*;
use std::net::TcpListener;

fn main() {
    let puerto = TcpListener::bind("127.0.0.1:9999").unwrap();
    let piscina = Piscina::new(2);

    for conexion in puerto.incoming() {
        let conexion = conexion.unwrap();
        piscina.execute(|| {
            tratar_conexion(conexion);
        });
    }
}
