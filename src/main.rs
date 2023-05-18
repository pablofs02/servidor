use servidor::{Opciones, iniciar_servidor_http};
use std::env::args;

fn main() {
    let argumentos: Vec<String> = args().collect();
    let mut opciones = Opciones::base();
    opciones.configurar(&argumentos);
    iniciar_servidor_http(opciones);
}
