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
//     hidden = new BuildVisibility(() -> false),
//     shown = new BuildVisibility(() -> true),
//     debugOnly = new BuildVisibility(() -> false),
//     editorOnly = new BuildVisibility(() -> Vars.state.rules.editor),
//     sandboxOnly = new BuildVisibility(() -> Vars.state == null || Vars.state.rules.infiniteResources),
//     campaignOnly = new BuildVisibility(() -> Vars.state == null || Vars.state.isCampaign()),
//     lightingOnly = new BuildVisibility(() -> Vars.state == null || Vars.state.rules.lighting || Vars.state.isCampaign()),
//     ammoOnly = new BuildVisibility(() -> Vars.state == null || Vars.state.rules.unitAmmo),
//     fogOnly = new BuildVisibility(() -> Vars.state == null || Vars.state.rules.fog || Vars.state.rules.editor);
    pub static ref HIDDEN: BuildVisibility = BuildVisibility::new(Box::new(|| false));
    pub static ref SHOWN: BuildVisibility = BuildVisibility::new(Box::new(|| true));
    pub static ref DEBUG_ONLY: BuildVisibility = BuildVisibility::new(Box::new(|| false));
    pub static ref EDITOR_ONLY: BuildVisibility = BuildVisibility::new(Box::new(|| unsafe{crate::vars::STATE.unwrap().rules.editor}));
    pub static ref SANDBOX_ONLY: BuildVisibility = BuildVisibility::new(Box::new(|| unsafe {crate::vars::STATE.is_none()} || unsafe {crate::vars::STATE.unwrap().rules.infinite_resources}));
    pub static ref CAMPAIGN_ONLY: BuildVisibility = BuildVisibility::new(Box::new(|| false));
    pub static ref LIGHTING_ONLY: BuildVisibility = BuildVisibility::new(Box::new(|| unsafe {crate::vars::STATE.is_none()} || unsafe {crate::vars::STATE.unwrap().rules.lighting} || unsafe {crate::vars::STATE.unwrap().is_campaign()}));


}