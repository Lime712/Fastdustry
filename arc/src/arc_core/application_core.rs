// use std::fs::File;
//
// use crate::arc_core::application_listener::ApplicationListener;
//
// pub struct ApplicationCore {
//     pub modules: Vec<dyn ApplicationListener>,
// }
//
// impl ApplicationCore {
//     fn setup(&self) {}
//
//     pub fn add(&mut self, module: Box<dyn ApplicationListener>) {
//         self.modules.push(module)
//     }
// }
//
// impl ApplicationListener for ApplicationCore {
//     fn init(&self) {
//         self.setup();
//
//         for listener in self.modules.iter() {
//             listener.init();
//         }
//     }
//
//     fn resize(&self) {
//         for listener in self.modules.iter() {
//             listener.resize();
//         }
//     }
//
//     fn update(&self) {
//         for listener in self.modules.iter() {
//             listener.update();
//         }
//     }
//
//     fn pause(&self) {
//         for listener in self.modules.iter() {
//             listener.pause();
//         }
//     }
//
//     fn resume(&self) {
//         for listener in self.modules.iter() {
//             listener.resume();
//         }
//     }
//
//     fn dispose(&self) {
//         for listener in self.modules.iter() {
//             listener.dispose();
//         }
//     }
//
//     fn exit(&self) {
//         for listener in self.modules.iter() {
//             listener.exit();
//         }
//     }
//
//     fn file_dropped(&self, fi: File) {
//         for listener in self.modules.iter() {
//             listener.file_dropped(&fi);
//         }
//     }
// }
