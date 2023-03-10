use crate::world::meta::stats::Stats;

pub trait MappableContent {
    fn get_name(&self) -> &str;
    fn to_string(&self) -> String {
        self.get_name().to_string()
    }
}

#[derive(Clone, Debug)]
pub struct UnlockableContent {
    /// Stat storage for this content. Initialized on demand.
    pub stats: Stats,
    /// Localized, formal name. Never null. Set to internal name if not found in bundle.
    pub localized_name: &'static str,
    /// Localized description & details. May be null.
    pub description: String,
    pub details: String,
    /// Whether this content is always unlocked in the tech tree.
    pub always_unlocked: bool,
    /// Whether to show the description in the research dialog preview.
    pub inline_description: bool,
    /// Whether details of blocks are hidden in custom games if they haven't been unlocked in campaign mode.
    pub hide_details: bool,
    /// If false, ALL icon generation is disabled for this content; createIcons is not called.
    pub generate_icons: bool,
    /// Special logic icon ID.
    pub icon_id: i32,
    /// How big the content appears in certain selection menus
    pub selection_size: f32,
    // /// Icon of the content to use in UI.
    // pub uiIcon: TextureRegion,
    // /// Icon of the full content. Unscaled.
    // pub fullIcon: TextureRegion,
    // /// The tech tree node for this content, if applicable. Null if not part of a tech tree.
    // pub techNode: TechNode,
    /// Unlock state. Loaded from settings. Do not modify outside of the constructor.
    pub unlocked: bool,
}

impl Default for UnlockableContent {
    fn default() -> Self {
        Self {
            stats: Stats::new(),
            localized_name: "unknown",
            description: String::default(),
            details: String::default(),
            always_unlocked: false,
            inline_description: false,
            hide_details: false,
            generate_icons: true,
            icon_id: 0,
            selection_size: 1.0,
            unlocked: false,
        }
    }
}

impl MappableContent for UnlockableContent {
    fn get_name(&self) -> &str {
        &self.localized_name
    }
}

impl UnlockableContent {
    fn new(name: &'static str) -> Self {
        Self {
            localized_name: name,
            ..Default::default()
        }
    }
    pub fn check_status(&mut self) {
        if self.stats.initialized {
            self.set_stats();
            self.stats.initialized = true;
        }
    }

    pub fn set_stats(&mut self) {}

    pub fn locked(&self) -> bool {
        !self.always_unlocked && !self.unlocked
    }
}
