use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::fs::{File, OpenOptions};
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::path::Path;
use std::time::UNIX_EPOCH;

use filepath::FilePath;

use crate::arc_core::files::FType;
use crate::arc_core::func::{Cons, cons};
use crate::arc_core::util::Disposable;

pub struct Fi {
    file: File,
    file_type: FType,
}

impl Fi {
    pub fn new(file: File, file_type: FType) -> Fi {
        Fi { file, file_type }
    }

    pub fn new_from_path(path: String, file_type: FType) -> Fi {
        let f = match File::create(path) {
            Ok(f) => f,
            Err(e) => panic!("Error opening file: {}", e),
        };
        Fi::new(f, file_type)
    }

    pub fn new_from_file(file: File) -> Fi {
        Fi::new(file, FType::Absolute)
    }

    pub fn new_from_path_and_type(path: String, file_type: FType) -> Fi {
        Fi::new_from_path(path, file_type)
    }

    pub fn get(path: String) -> Fi {
        Fi::new_from_path(path, FType::Absolute)
    }

    pub fn empty_directory(file: File, preserver_tree: bool) {
        let path = Path::new(&file.path().unwrap().to_str().unwrap().to_string());
        if path.exists() {
            let mut files = path.read_dir().unwrap();
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

    pub fn path(&file: &'static File) -> &'static Path {
        Path::new(&file.path().unwrap().to_str().unwrap().to_string())
    }

    pub fn copy_file(source: Fi, destination: Fi) {
        let source_path = Fi::path(&source.file);
        let destination_path = Fi::path(&destination.file);
        if source_path.exists() {
            fs::copy(source_path, destination_path).unwrap();
        } else {
            panic!("Source file does not exist!");
        }
    }

    pub fn copy_directory(source: Fi, destination: Fi) {
        let source_path = Fi::path(&source.file);
        let destination_path = Fi::path(&destination.file);
        if source_path.exists() {
            fs::copy(source_path, destination_path).unwrap();
        } else {
            panic!("Source file does not exist!");
        }
    }

    pub fn absolute_path(&self) -> String {
        self.file.path().unwrap().to_str().unwrap().to_string().replace("\\", "/")
    }

    pub fn exists(&self) -> bool {
        Fi::path(&self.file).exists()
    }

    pub fn is_directory(&self) -> bool {
        Fi::path(&self.file).is_dir()
    }

    pub fn is_file(&self) -> bool {
        Fi::path(&self.file).is_file()
    }

    pub fn name(&self) -> String {
        Fi::path(&self.file).file_name().unwrap().to_str().unwrap().to_string()
    }

    pub fn extension(&self) -> String {
        Fi::path(&self.file).extension().unwrap().to_str().unwrap().to_string()
    }

    pub fn ext_equals(&self, ext: String) -> bool {
        self.extension() == ext
    }

    pub fn name_without_extension(&self) -> String {
        Fi::path(&self.file).file_stem().unwrap().to_str().unwrap().to_string()
    }

    pub fn path_without_extension(&self) -> String {
        let mut path = self.absolute_path();
        path.replace(&format!(".{}", self.extension()), "")
    }

    pub fn file_type(&self) -> FType {
        self.file_type.clone()
    }

    pub fn read(&mut self) -> String {
        let mut contents = String::new();
        self.file.read_to_string(&mut contents).unwrap();
        contents
    }

    pub fn read_with_charset(&mut self, charset: String) -> String {
        todo!("Implement read_with_charset")
    }

    pub fn read_bytes(&mut self) -> Vec<u8> {
        let mut contents = Vec::new();
        self.file.read_to_end(&mut contents).unwrap();
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
            self.file.write_all(contents.as_bytes()).unwrap();
        }
    }

    pub fn write_charset(&mut self, contents: String, charset: String, append: bool) {
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
            self.file.write_all(contents.as_slice()).unwrap();
        }
    }

    pub fn walk(&mut self, &cons: &'static Box<dyn Cons<&mut Fi>>) {
        if self.is_directory() {
            let mut files = Fi::path(&self.file).read_dir().unwrap();
            for f in files {
                let f = f.unwrap();
                let mut fi = Fi::new_from_path(f.path().to_str().unwrap().to_string(), self.file_type.clone());
                fi.walk(&cons);
            }
        } else {
            cons.get(self);
        }
    }

    /// Recursively iterates through all files in this directory and adds them to an array.
    pub fn find_all(&mut self) -> Vec<Fi> {
        let mut files = Vec::new();
        let b = cons(|fi: &mut Fi| {
            files.push(fi.clone());
        });
        self.walk(&b);
        files
    }

    /// Returns a handle to the child with the specified name.
    pub fn child(&mut self, name: String) -> Fi {
        Fi::new_from_path(format!("{}/{}", self.absolute_path(), name), self.file_type.clone())
    }

    pub fn sibling(&mut self, name: String) -> Fi {
        Fi::new_from_path(format!("{}/{}", self.absolute_path().replace(&self.name(), ""), name), self.file_type.clone())
    }


    /// Returns the paths to the children of this directory. Returns an empty list if this file handle represents a file and not a
    /// directory. On the desktop, an {@link FileType#internal} handle to a directory on the classpath will return a zero length
    /// array.
    pub fn list(&mut self) -> Vec<Fi> {
        let mut files = Vec::new();
        if self.is_directory() {
            let mut fs = Fi::path(&self.file).read_dir().unwrap();
            for f in fs {
                let f = f.unwrap();
                files.push(Fi::new_from_path(f.path().to_str().unwrap().to_string(), self.file_type.clone()));
            }
        }
        files
    }

    pub fn parent(&mut self) -> Fi {
        Fi::new_from_path(self.absolute_path().replace(&self.name(), ""), self.file_type.clone())
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
        let mut files = self.list();
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

    pub fn last_modified(&mut self) -> u64 {
        let metadata = fs::metadata(self.absolute_path()).unwrap();
        metadata.modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}

impl Clone for Fi {
    fn clone(&self) -> Fi {
        Fi {
            file: File::create(self.absolute_path()).unwrap(),
            file_type: self.file_type.clone(),
        }
    }
}

impl Debug for Fi {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fi {{ file: {:?}, file_type: {:?} }}", self.file, self.file_type)
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