use std::process::exit;

#[derive(Debug, Copy, Clone)]
pub struct Opciones {
    pub local: bool,
    pub verboso: bool,
    pub puerto: u16
}

impl Opciones {
    #[must_use]
    pub const fn base() -> Self {
        Self {
            local: true,
            verboso: false,
            puerto: 9999
        }
    }

    pub fn configurar(&mut self, argumentos: &[String]) {
        for argumento in argumentos {
            // Saltar nombre del comando o cadena vacía
            if &argumento[..] == argumentos.get(0).unwrap() || argumento.is_empty() {
                continue;
            }
            // Formato UNIX '-v' o BSD 'v'
            if let Some(argumento) = argumento.strip_prefix('-') {
                self.tratar_caracteres(argumento);
            } else {
                self.tratar_caracteres(argumento);
            }
        }
    }

    fn tratar_caracteres(&mut self, caracteres: &str) {
        for letra in caracteres.chars() {
            match letra {
                'o' => {
                    mensaje_de_ayuda();
                    exit(0);
                }
                'l' => self.local = true,
                'p' => self.local = false,
                'v' => self.verboso = true,
                _ => {
                    eprintln!("Argumento desconocido: '{letra}'");
                    mensaje_de_ayuda();
                    exit(22);
                }
            }
        }
    }
}

fn mensaje_de_ayuda() {
    println!(
        "Modo de empleo: servidor [OPCIONES...]
    o    mostrar opciones
    l    servidor local (por defecto)
    p    servidor público
    v    más información"
    );
}
