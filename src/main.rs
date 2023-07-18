use servus::{iniciar_servidor_http, Opciones};
use std::env::args;

fn main() {
    let argumentos: Vec<String> = args().collect();
    let mut opciones = Opciones::default();
    opciones.configurar(&argumentos);
    iniciar_servidor_http(opciones);
}
