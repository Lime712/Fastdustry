use lazy_static::lazy_static;

pub struct BuildVisibility {
    is_visible: Box<dyn Fn() -> bool + 'static + Sync>,
}

impl BuildVisibility {
    pub fn new(is_visible: Box<dyn Fn() -> bool>) -> Self {
        Self { is_visible }
    }

    pub fn is_visible(&self) -> bool {
        (self.is_visible)()
    }
}

lazy_static! {
    pub static ref HIDDEN: BuildVisibility = BuildVisibility::new(Box::new(|| false));
    pub static ref SHOWN: BuildVisibility = BuildVisibility::new(Box::new(|| true));
    pub static ref DEBUG_ONLY: BuildVisibility = BuildVisibility::new(Box::new(|| false));
    pub static ref EDITOR_ONLY: BuildVisibility = BuildVisibility::new(Box::new(|| unsafe{crate::vars::STATE.unwrap().rules.editor}));
    pub static ref SANDBOX_ONLY: BuildVisibility = BuildVisibility::new(Box::new(|| unsafe {crate::vars::STATE.is_none()} || unsafe {crate::vars::STATE.unwrap().rules.infinite_resources}));
    pub static ref CAMPAIGN_ONLY: BuildVisibility = BuildVisibility::new(Box::new(|| false));
    pub static ref LIGHTING_ONLY: BuildVisibility = BuildVisibility::new(Box::new(|| unsafe {crate::vars::STATE.is_none()} || unsafe {crate::vars::STATE.unwrap().rules.lighting} || unsafe {crate::vars::STATE.unwrap().is_campaign()}));
}