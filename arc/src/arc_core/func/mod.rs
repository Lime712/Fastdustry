use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

pub trait Cons<T> {
    fn get(self, t: T);
}

impl<T, F: Fn(T)> Cons<T> for F {
    fn get(self, t: T) {
        self(t)
    }
}

pub fn cons<T, F: Fn(T)>(f: F) -> Box<dyn Cons<T>> {
    Box::new(f)
}

pub struct FloatP {
    pub get: Arc<Mutex<dyn Fn() -> f64 + Send + 'static>>,
}

impl FloatP {
    pub fn new(float_p: impl Fn() -> f64 + Send + 'static) -> Self {
        let boxed = Box::new(float_p);
        let arc = Arc::new(Mutex::new(boxed));
        Self { get: arc }
    }

    pub fn run(&self) -> f64 {
        let arc = self.get.clone();
        let mut lock = arc.lock().unwrap();
        let boxed = lock.deref_mut();
        (boxed)()
    }
}