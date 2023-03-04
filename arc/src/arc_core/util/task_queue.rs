use std::collections::HashSet;
use std::hash::Hash;

pub struct TaskQueue {
    runnables: HashSet<Runnable>,
    executed_runnables: HashSet<Option<Runnable>>,
}

impl TaskQueue {
    pub fn new() -> Self {
        Self {
            runnables: HashSet::new(),
            executed_runnables: HashSet::new(),
        }
    }

    pub fn run(&mut self) {
        self.executed_runnables.clear();
        for runnable in self.runnables.iter() {
            (runnable.f)();
            // self.executed_runnables.insert(Some(*unnable));
        }
    }

    pub fn clear(&mut self) {
        self.runnables.clear();
    }

    pub fn size(&self) -> usize {
        self.runnables.len()
    }

    pub fn post(&mut self, runnable: Box<dyn Fn()>) {
        self.runnables.insert(Runnable::new(runnable));
    }
}

static mut counter: usize = 0;

pub struct Runnable {
    id: usize,
    pub f: Box<dyn Fn()>,
}

impl Runnable {
    pub fn new(f: Box<dyn Fn()>) -> Self {
        unsafe {
            counter += 1;
            Self { id: counter, f }
        }
    }
}

impl PartialEq for Runnable {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Runnable {}

impl Hash for Runnable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
//
// impl Clone for Runnable {
//     fn clone(&self) -> Self {
//         Self { id: self.id, f: self.f.clone() }
//     }
// }