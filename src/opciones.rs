#[derive(Debug, Copy, Clone)]
pub struct Opciones {
    pub ayuda: bool,
    pub local: bool,
    pub verboso: bool
}

impl Opciones {
    pub fn base() -> Opciones {
        Self {
            ayuda: false,
            local: true,
            verboso: false
        }
    }
}
