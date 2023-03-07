use crate::world::meta::stats::Stats;

pub struct UnlockableContent {
    /// Stat storage for this content. Initialized on demand.
    pub stats: Stats,
    /// Localized, formal name. Never null. Set to internal name if not found in bundle.
    pub localizedName: String,
    /// Localized description & details. May be null.
    pub description: String,
    pub details: String,
    /// Whether this content is always unlocked in the tech tree.
    pub alwaysUnlocked: bool,
    /// Whether to show the description in the research dialog preview.
    pub inlineDescription: bool,
    /// Whether details of blocks are hidden in custom games if they haven't been unlocked in campaign mode.
    pub hideDetails: bool,
    /// If false, ALL icon generation is disabled for this content; createIcons is not called.
    pub generateIcons: bool,
    /// Special logic icon ID.
    pub iconId: i32,
    /// How big the content appears in certain selection menus
    pub selectionSize: f32,
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
            localizedName: String::default(),
            description: String::default(),
            details: String::default(),
            alwaysUnlocked: false,
            inlineDescription: false,
            hideDetails: false,
            generateIcons: true,
            iconId: 0,
            selectionSize: 1.0,
            unlocked: false,
        }
    }
}