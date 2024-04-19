use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Orden = Box<dyn FnOnce() + Send + 'static>;

struct Hebra {
    hilo: Option<thread::JoinHandle<()>>,
}

impl Hebra {
    fn new(instructor: Arc<Mutex<mpsc::Receiver<Orden>>>) -> Self {
        let hilo = thread::spawn(move || loop {
            let mensaje = instructor.lock().unwrap().recv();
            match mensaje {
                Ok(hilo) => {
                    hilo();
                }
                Err(_) => {
                    // si rompe ya no recibe???
                    // break;
                }
            }
        });
        Self { hilo: Some(hilo) }
    }
}

pub struct Piscina {
    hebras: Vec<Hebra>,
    instructor: Option<mpsc::Sender<Orden>>,
}

impl Piscina {
    #[must_use]
    pub fn new(capacidad: usize) -> Self {
        let (emisor, receptor) = mpsc::channel();
        let receptor = Arc::new(Mutex::new(receptor));
        let mut hebras = Vec::with_capacity(capacidad);
        for _ in 0..capacidad {
            hebras.push(Hebra::new(Arc::clone(&receptor)));
        }
        Self {
            hebras,
            instructor: Some(emisor),
        }
    }

    pub fn ejecutar<F>(&self, orden: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.instructor
            .as_ref()
            .unwrap()
            .send(Box::new(orden))
            .unwrap_or_else(|_| {
                // aquí es donde crashea cuando se lanza desde vim
            });
    }
}

impl Drop for Piscina {
    fn drop(&mut self) {
        drop(self.instructor.take());
        for hebra in &mut self.hebras {
            if let Some(hilo) = hebra.hilo.take() {
                hilo.join().expect("Error al dropear");
            }
        }
    }
}
