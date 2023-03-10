use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

pub struct Cons<T> {
    function: Arc<Mutex<dyn Fn(T) + Send + 'static>>,
}

impl<T> Cons<T> {
    pub fn new(function: impl Fn(T) + Send + 'static) -> Self {
        let boxed = Box::new(function);
        let arc = Arc::new(Mutex::new(boxed));
        Self { function: arc }
    }

    pub fn run(&self, t: T) {
        let arc = self.function.clone();
        let mut lock = arc.lock().unwrap();
        let boxed = lock.deref_mut();
        (boxed)(t)
    }
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
