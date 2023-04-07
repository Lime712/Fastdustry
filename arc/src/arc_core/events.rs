use std::any::Any;
use std::collections::HashMap;

static mut EVENTS: Option<HashMap<dyn Any, Vec<dyn Fn()>>> = None;

// todo: fix this
pub struct Events {}
