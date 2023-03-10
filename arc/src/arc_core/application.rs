use std::collections::HashSet;

use crate::arc_core::application_listener::ApplicationListener;

pub trait Application {
    /// Returns a list of ALL the application listeners used.
    fn get_listeners(&mut self) -> &mut HashSet<Box<dyn ApplicationListener>>;

    /// Adds a new application listener.
    fn add_listener(&mut self, listener: Box<dyn ApplicationListener>) {
        self.get_listeners().insert(listener);
    }

    /// Removes an application listener.
    fn remove_listeners(&mut self, listener: Box<dyn ApplicationListener>) {
        self.get_listeners().remove(&*listener);
    }

    /// Call this before update() in each backend.
    fn default_update(&self) {
        // Core.settings.autosave();
        // Time.update_gobal();
    }

    /// return what this application has, e.g. Android or Desktop
    fn get_type(&self) -> ApplicationType;

    fn is_desktop(&self) -> bool {
        self.get_type() == ApplicationType::Desktop
    }

    fn is_headless(&self) -> bool {
        self.get_type() == ApplicationType::Headless
    }

    fn is_android(&self) -> bool {
        self.get_type() == ApplicationType::Android
    }

    fn is_ios(&self) -> bool {
        self.get_type() == ApplicationType::IOS
    }

    fn is_mobile(&self) -> bool {
        self.get_type() == ApplicationType::Android || self.get_type() == ApplicationType::IOS
    }

    fn is_web(&self) -> bool {
        self.get_type() == ApplicationType::Web
    }

    /// return the Android API level on Android, the major OS version on iOS (5, 6, 7, ..), or 0 on the Desktop.
    fn get_version(&self) -> i32 {
        0
    }

    /// return the rust heap memory usage in bytes
    fn get_memory_usage(&self) -> i64;

    fn get_clipboard_text(&self) -> String;

    fn set_clipboard_text(&self, text: String);

    /// open a folder in the system's file browser.
    /// * returns true if the folder was opened successfully.
    fn open_folder(&self, _path: String) -> bool {
        false
    }

    /// Launches the default browser to display a URI. If the default browser is not able to handle the specified URI, the
    /// application registered for handling URIs of the specified type is invoked. The application is determined from the protocol
    /// and path of the URI. A best effort is made to open the given URI; however, since external applications are involved, no guarantee
    /// can be made as to whether the URI was actually opened. If it is known that the URI was not opened, false will be returned;
    /// otherwise, true will be returned.
    /// return false if it is known the uri was not opened, true otherwise
    fn open_uri(&self, _uri: String) -> bool {
        false
    }

    /// Posts a runnable on the main loop thread
    fn post_runnable(&self);

    fn exit(&mut self);

    fn dispose(&self) {
        // // flush any changes to settings upon dispose
        // if Core.settings != null {
        //     Core.settings.autosave();
        // }
        //
        // if Core.audio != null {
        //     Core.audio.dispose();
        // }
    }
}

#[derive(PartialEq)]
pub enum ApplicationType {
    Desktop,
    Headless,
    Android,
    IOS,
    Web,
}
