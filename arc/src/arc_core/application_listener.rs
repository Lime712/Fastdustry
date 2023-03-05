use std::fs::File;
use std::hash::{Hash, Hasher};

/// An <code>ApplicationListener</code> is called when the [Application] is created, resumed, rendering, paused or destroyed.
/// All methods are called in a thread that has the OpenGL context current. You can thus safely create and manipulate graphics
/// resources.
/// </p>
///
/// <p>
/// The <code>ApplicationListener</code> interface follows the standard Android activity life-cycle and is emulated on the desktop
/// accordingly.
/// </p>
/// **author:** mzechner
pub trait ApplicationListener {
    /// Called when the [Application] is first created.
    /// Only gets called if the application is created before the listener is added.
    fn init(&self) {}

    fn resize(&self) {}

    fn update(&self) {}

    fn pause(&self) {}

    fn resume(&self) {}

    fn dispose(&self) {}

    fn exit(&self) {}

    fn file_dropped(&self, fi: &File) {}
}

impl PartialEq<Self> for dyn ApplicationListener {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl Eq for dyn ApplicationListener {}

impl Hash for dyn ApplicationListener {
    fn hash<H: Hasher>(&self, state: &mut H) {
        std::ptr::hash(self, state)
    }
}

impl Clone for Box<dyn ApplicationListener> {
    fn clone(&self) -> Self {
        self.clone()
    }
}