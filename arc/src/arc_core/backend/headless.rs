use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use crate::arc_core::application::{Application, ApplicationType};
use crate::arc_core::application_listener::ApplicationListener;

use crate::arc_core::util::task_queue::TaskQueue;
use crate::arc_core::util::time::nanos;
use crate::debug;

pub struct HeadlessApplication {
    // graphics: MockGraphics,
    listeners: HashSet<Box<dyn ApplicationListener>>,
    pub runnables: TaskQueue,
    // exception_handler: Option<dyn Cons<Exception>>,
    render_interval: i64,
    pub main_loop_thread: JoinHandle<()>,
    running: bool,
}

impl Application for HeadlessApplication {
    fn get_listeners(&mut self) -> &mut HashSet<Box<dyn ApplicationListener>> {
        &mut self.listeners
    }

    fn get_type(&self) -> ApplicationType {
        ApplicationType::Headless
    }

    fn get_memory_usage(&self) -> i64 {
        todo!()
    }

    fn get_clipboard_text(&self) -> String {
        "".to_string()
    }

    fn set_clipboard_text(&self, _text: String) {
        // do nothing
    }

    fn post_runnable(&self) {
        // self.runnables.add_task(Task::new(runnable));
    }

    fn exit(&mut self) {
        self.running = false;
    }
}

impl HeadlessApplication {
    pub fn start(listener: Box<dyn ApplicationListener>, render_interval_sec: f64) {
        let mut ri: i64 = 0;
        if render_interval_sec > 0.0 {
            ri = (render_interval_sec * 1000000000f64) as i64;
        } else if render_interval_sec < 0.0 {
            ri = -1;
        }
        let mut h = HeadlessApplication {
            // graphics: MockGraphics::new(),
            listeners: HashSet::new(),
            runnables: TaskQueue::new(),
            // exception_handler: None,
            render_interval: ri,
            main_loop_thread: thread::Builder::new()
                .name("HeadlessApplication".to_string())
                .spawn(move || {
                    // do nothing
                })
                .unwrap(),
            running: true,
        };
        h.add_listener(listener);
        debug!("listeners len: {}", h.listeners.len());
        // unsafe {
        // crate::arc_core::core::APP = Some(&h);
        // }
        HeadlessApplication::initialize(Arc::new(Mutex::new(h)));
    }

    pub fn initialize(this: Arc<Mutex<HeadlessApplication>>) {
        thread::Builder::new()
            .name("HeadlessApplication".to_string())
            .spawn(move || {
                this.lock().unwrap().main_loop();
            })
            .unwrap()
            .join()
            .expect("CRASHED");
        // self.main_loop_thread.join().unwrap();
    }

    fn main_loop(&mut self) {
        debug!("start main_loop()");
        debug!("init listeners len: {}", self.listeners.len());
        for listener in self.listeners.iter() {
            listener.init();
        }

        let mut t = nanos() + self.render_interval;
        if self.render_interval >= 0 {
            while self.running {
                // debug!("main_loop()");
                let n = nanos();
                if t > n {
                    let sleep_time = t - n;
                    thread::sleep(std::time::Duration::from_nanos(sleep_time as u64));

                    t += self.render_interval;
                } else {
                    t = n + self.render_interval;
                }

                self.runnables.run();
                // graphics.incrementFrameId();
                self.default_update();

                for listener in self.listeners.iter() {
                    listener.update();
                }
                // graphics.updateTime();

                // If one of the runnables set running to false, for example after an exit()
                if !self.running {
                    break;
                }
            }
        }

        for listener in self.listeners.iter() {
            listener.pause();
            listener.dispose();
        }
        self.dispose();
    }
}

unsafe impl Send for HeadlessApplication {}
