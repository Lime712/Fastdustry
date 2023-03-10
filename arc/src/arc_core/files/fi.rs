use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::fs::{File, OpenOptions};
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use filepath::FilePath;

use crate::arc_core::files::FType;
use crate::{debug, trace};

// todo: error handling is currently not implemented
pub struct Fi {
    pub path: String,
    pub file_type: FType,
}

impl Fi {
    pub fn new(file: File, file_type: FType) -> Fi {
        let path = file.path().unwrap().to_str().unwrap().to_string();
        Fi { file_type, path }
    }

    pub fn new_from_path_and_type(path: String, file_type: FType) -> Fi {
        Fi { file_type, path }
    }

    pub fn new_from_file(file: File) -> Fi {
        Fi::new(file, FType::Absolute)
    }

    pub fn new_from_path(path: String) -> Fi {
        Fi::new_from_path_and_type(path, FType::Absolute)
    }

    pub fn get(path: String) -> Fi {
        Fi::new_from_path_and_type(path, FType::Absolute)
    }

    pub fn empty_directory(file: File, preserver_tree: bool) {
        let binding = file.path().unwrap().to_str().unwrap().to_string();
        let path = Path::new(&binding);
        if path.exists() {
            let files = path.read_dir().unwrap();
            for f in files {
                let f = f.unwrap();
                if !f.path().is_dir() {
                    fs::remove_file(f.path()).unwrap();
                } else if preserver_tree {
                    Fi::empty_directory(File::create(f.path()).unwrap(), true);
                } else {
                    Fi::delete_directory(File::create(f.path()).unwrap());
                }
            }
        }
    }

    pub fn delete_directory(file: File) {
        let path = Fi::path(&file);
        if path.exists() {
            Fi::empty_directory(file, false);
            fs::remove_dir(path).unwrap();
        }
    }

    pub fn path(file: &File) -> PathBuf {
        // debug!("Getting path from file: {:?}", file);
        Path::new(&file.path().unwrap().to_str().unwrap().to_string()).to_path_buf()
    }

    pub fn copy_file(source: Fi, destination: Fi) {
        let source_path = Fi::path(&source.file());
        let destination_path = Fi::path(&destination.file());
        if source_path.exists() {
            fs::copy(source_path, destination_path).unwrap();
        } else {
            panic!("Source file does not exist!");
        }
    }

    pub fn file_option(&self) -> std::io::Result<File> {
        // debug!("Opening file: {}", self.path);
        let path = Path::new(&self.path);
        if path.is_dir() {
            // debug!("File is a directory: {}", self.path);
            return Ok(File::open(self.path.clone()).unwrap());
        }
        OpenOptions::new().write(true).open(self.path.clone())
    }

    pub fn file(&self) -> File {
        match self.file_option() {
            Ok(file) => file,
            Err(e) => {
                // debug!("{:?}", e);
                panic!("File does not exist! {e}");
            }
        }
    }

    pub fn copy_directory(source: Fi, destination: Fi) {
        let source_path = Fi::path(&source.file());
        let destination_path = Fi::path(&destination.file());
        if source_path.exists() {
            fs::copy(source_path, destination_path).unwrap();
        } else {
            panic!("Source file does not exist!");
        }
    }

    pub fn absolute_path(&self) -> String {
        match self.file_option() {
            Ok(_) => {
                // debug!("File exists: {}", self.path);
                Fi::path(&self.file())
                    .canonicalize()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .replace('\\', "/")
            }
            Err(_e) => {
                // debug!("absolute path error: {:?}", e);
                self.path.clone()
            }
        }
    }

    pub fn exists(&self) -> bool {
        match self.file_option() {
            Ok(_) => {
                // debug!("File exists: {}", self.path);
                Fi::path(&self.file()).exists()
            }
            Err(e) => {
                trace!("File does not exist: {}, {}", self.path, e);
                false
            }
        }
    }

    pub fn is_directory(&self) -> bool {
        Path::new(&self.path).is_dir()
    }

    pub fn is_file(&self) -> bool {
        Fi::path(&self.file()).is_file()
    }

    pub fn name(&self) -> String {
        Fi::path(&self.file())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn extension(&self) -> String {
        Fi::path(&self.file())
            .extension()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn ext_equals(&self, ext: String) -> bool {
        self.extension() == ext
    }

    pub fn name_without_extension(&self) -> String {
        Fi::path(&self.file())
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn path_without_extension(&self) -> String {
        let path = self.absolute_path();
        path.replace(&format!(".{}", self.extension()), "")
    }

    pub fn file_type(&self) -> FType {
        self.file_type
    }

    pub fn read(&mut self) -> String {
        let mut contents = String::new();
        self.file().read_to_string(&mut contents).unwrap();
        contents
    }

    pub fn read_with_charset(&mut self, _charset: String) -> String {
        todo!("Implement read_with_charset")
    }

    pub fn read_bytes(&mut self) -> Vec<u8> {
        let mut contents = Vec::new();
        self.file().read_to_end(&mut contents).unwrap();
        contents
    }

    pub fn length(&mut self) -> usize {
        self.read_bytes().len()
    }

    pub fn write(&mut self, contents: String, append: bool) {
        if append {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(self.absolute_path())
                .unwrap();

            if let Err(e) = file.write(contents.as_bytes()) {
                eprintln!("Couldn't write to file: {}", e);
            }
        } else {
            self.file().write_all(contents.as_bytes()).unwrap();
        }
    }

    pub fn write_charset(&mut self, _contents: String, _charset: String, _append: bool) {
        todo!("Implement write_charset")
    }

    pub fn write_bytes(&mut self, contents: Vec<u8>, append: bool) {
        if append {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(self.absolute_path())
                .unwrap();

            if let Err(e) = file.write(contents.as_slice()) {
                eprintln!("Couldn't write to file: {}", e);
            }
        } else {
            self.file().write_all(contents.as_slice()).unwrap();
        }
    }

    // pub fn walk(& mut self, cons: &Cons<&'static mut Fi>) {
    //     if self.is_directory() {
    //         let files = Fi::path(&self.file).read_dir().unwrap();
    //         for f in files {
    //             let f = f.unwrap();
    //             let mut fi = Fi::new_from_path_and_type(f.path().to_str().unwrap().to_string(), self.file_type.clone());
    //             fi.walk(&cons);
    //         }
    //     } else {
    //         let x = self.clone();
    //         let mut f = Fi::new(x.file.try_clone().unwrap(), x.file_type.clone());
    //         // cons.run(&mut f);
    //     }
    // }

    /// Recursively iterates through ALL files in this directory and adds them to an array.
    pub fn find_all(&mut self) -> Vec<Fi> {
        let mut files = Vec::new();
        if self.is_directory() {
            let fs = Fi::path(&self.file()).read_dir().unwrap();
            for f in fs {
                let f = f.unwrap();
                let mut fi = Fi::new_from_path_and_type(
                    f.path().to_str().unwrap().to_string(),
                    self.file_type,
                );
                files.append(&mut fi.find_all());
            }
        } else {
            files.push(self.clone());
        }
        files
    }

    /// Returns a handle to the child with the specified name.
    pub fn child(&mut self, name: String) -> Fi {
        Fi::new_from_path_and_type(format!("{}/{}", self.path, name), self.file_type)
    }

    pub fn sibling(&mut self, name: String) -> Fi {
        Fi::new_from_path_and_type(
            format!(
                "{}/{}",
                self.absolute_path().replace(&self.name(), ""),
                name
            ),
            self.file_type,
        )
    }

    /// Returns the paths to the children of this directory. Returns an empty list if this file handle represents a file and not a
    /// directory. On the desktop, an {@link FileType#internal} handle to a directory on the classpath will return a zero length
    /// array.
    pub fn list(&mut self) -> Vec<Fi> {
        let mut files = Vec::new();
        if self.is_directory() {
            let fs = Path::new(&self.path.clone()).read_dir().unwrap();
            for f in fs {
                let f = f.unwrap();
                files.push(Fi::new_from_path_and_type(
                    f.path().to_str().unwrap().to_string(),
                    self.file_type,
                ));
            }
        }
        files
    }

    pub fn parent(&mut self) -> Fi {
        Fi::new_from_path_and_type(
            self.absolute_path().replace(&self.name(), ""),
            self.file_type,
        )
    }

    pub fn create_directory(&mut self) {
        let p = match File::open(self.clone().path) {
            Ok(_p) => self.absolute_path(),
            Err(_) => self.path.clone(),
        };
        // debug!("creating directory: {}", p.clone() + "/");
        fs::create_dir(p).unwrap();
    }

    pub fn mkdirs(&mut self) {
        fs::create_dir_all(self.absolute_path()).unwrap();
    }

    pub fn delete(&mut self) {
        fs::remove_file(self.absolute_path()).unwrap();
    }

    pub fn delete_recursive(&mut self) {
        fs::remove_dir_all(self.absolute_path()).unwrap();
    }

    pub fn delete_directory_contents(&mut self) {
        let files = self.list();
        for mut f in files {
            f.delete_recursive();
        }
    }

    pub fn copy_to(&mut self, dest: Fi) {
        fs::copy(self.absolute_path(), dest.absolute_path()).unwrap();
    }

    pub fn move_to(&mut self, dest: Fi) {
        fs::rename(self.absolute_path(), dest.absolute_path()).unwrap();
    }

    pub fn last_modified(&self) -> u64 {
        let metadata = fs::metadata(self.absolute_path()).unwrap();
        metadata
            .modified()
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

impl Clone for Fi {
    fn clone(&self) -> Fi {
        Fi {
            file_type: self.file_type,
            path: self.path.clone(),
        }
    }
}

impl Debug for Fi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Fi {{ file: {:?}, file_type: {:?} }}",
            self.file(),
            self.file_type
        )
    }
}

impl PartialEq for Fi {
    fn eq(&self, other: &Self) -> bool {
        self.absolute_path() == other.absolute_path()
    }
}

impl Eq for Fi {}

impl Hash for Fi {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.absolute_path().hash(state);
    }
}

impl Display for Fi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.absolute_path())
    }
}
