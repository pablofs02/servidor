use std::sync::{mpsc, Arc, Mutex};
use std::thread;

#[derive(Debug, Copy, Clone)]
pub struct Opciones {
    pub ayuda: bool,
    pub local: bool,
    pub verboso: bool
}

type Orden = Box<dyn FnOnce() + Send + 'static>;

struct Nadador {
    movimiento: Option<thread::JoinHandle<()>>
}

impl Nadador {
    fn new(instructor: Arc<Mutex<mpsc::Receiver<Orden>>>) -> Self {
        let movimiento = thread::spawn(move || loop {
            let mensaje = instructor.lock().unwrap().recv();
            match mensaje {
                Ok(movimiento) => {
                    movimiento();
                }
                Err(_) => {
                    break;
                }
            }
        });
        Self { movimiento: Some(movimiento) }
    }
}

pub struct Piscina {
    nadadores: Vec<Nadador>,
    instructor: Option<mpsc::Sender<Orden>>
}

impl Piscina {
    #[must_use]
    pub fn new(capacidad: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut nadadores = Vec::with_capacity(capacidad);
        for _ in 0..capacidad {
            nadadores.push(Nadador::new(Arc::clone(&receiver)));
        }
        Self { nadadores, instructor: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where F: FnOnce() + Send + 'static {
        let orden = Box::new(f);
        self.instructor.as_ref().unwrap().send(orden).unwrap();
    }
}

impl Drop for Piscina {
    fn drop(&mut self) {
        drop(self.instructor.take());
        for nadador in &mut self.nadadores {
            if let Some(movimiento) = nadador.movimiento.take() {
                movimiento.join().expect("Mal movimiento");
            }
        }
    }
}
