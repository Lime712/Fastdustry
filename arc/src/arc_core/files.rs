//
// /// Provides standard access to the filesystem, classpath, Android SD card, and Android assets directory.
// pub trait Files {
//     /// Returns a handle representing a file or directory.
//     fn get(&self, path: String, f_type: FileType) -> Fi;
//
//     /// Convenience method that returns a {@link FileType#classpath} file handle.
//     fn internal(&self, path: String) -> Fi {
//         self.get(path, FileType::classpath)
//     }
//
//     /// Convenience method that returns a {@link FileType#internal} file handle.
//     fn classpath(&self, path: String) -> Fi {
//         self.get(path, FileType::internal)
//     }
//
//     /// Convenience method that returns a {@link FileType#external} file handle.
//     fn external(&self, path: String) -> Fi {
//         self.get(path, FileType::external)
//     }
//
//     /// Convenience method that returns a {@link FileType#absolute} file handle.
//     fn absolute(&self, path: String) -> Fi {
//         self.get(path, FileType::absolute)
//     }
//
//     /// Convenience method that returns a {@link FileType#local} file handle.
//     fn local(&self, path: String) -> Fi {
//         self.get(path, FileType::local)
//     }
//
//     /// return absolute path to cache directory
//     fn get_cache_path(&self) -> String {
//         local("cache").absolute_path()
//     }
//
//     /// Convenience method that returns a {@link FileType#local} file handle for the cache directory.
//     fn get_cache_dir(&self) -> Fi {
//         local("cache")
//     }
//
//     /// the external storage path directory. This is the SD card on Android and the home directory of the current user on
//     /// the desktop.
//     fn get_external_storage_path(&self) -> String;
//
//     /// true if the external storage is ready for file IO. Eg, on Android, the SD card is not available when mounted for use
//     /// with a PC.
//     fn is_external_storage_available(&self) -> bool;
//
//     /// the local storage path directory. This is the private files directory on Android and the directory of the jar on the
//     /// desktop.
//     fn get_local_storage_path(&self) -> String;
//
//     /// true if the local storage is ready for file IO.
//     fn is_local_storage_available(&self) -> bool;
// }
//
/// Indicates how to resolve a path to a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FType {
    Classpath,
    Internal,
    External,
    Absolute,
    Local,
}
pub mod fi;
