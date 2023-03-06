use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

use json::{JsonValue, object};

use crate::arc_core::core::SETTINGS;
use crate::arc_core::files::fi::Fi;
use crate::arc_core::files::FType;
use crate::arc_core::util::log::get_current_time_string;
use crate::arc_core::util::os::get_app_data_directory;
use crate::arc_core::util::time::millis;
use crate::debug;

const TYPE_BOOL: u8 = 0;
const TYPE_INT: u8 = 1;
const TYPE_LONG: u8 = 2;
const TYPE_FLOAT: u8 = 3;
const TYPE_STRING: u8 = 4;
const TYPE_BINARY: u8 = 5;
const MAX_BACKUPS: usize = 10;

pub enum Value {
    Bool(bool),
    Int(i32),
    Long(i64),
    Float(f32),
    String(String),
    Binary(Vec<u8>),
}

impl Clone for Value {
    fn clone(&self) -> Value {
        match self {
            Value::Bool(b) => Value::Bool(*b),
            Value::Int(i) => Value::Int(*i),
            Value::Long(l) => Value::Long(*l),
            Value::Float(f) => Value::Float(*f),
            Value::String(s) => Value::String(s.clone()),
            Value::Binary(b) => Value::Binary(b.clone()),
        }
    }
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Bool(b) => b.to_string(),
            Value::Int(i) => i.to_string(),
            Value::Long(l) => l.to_string(),
            Value::Float(f) => f.to_string(),
            Value::String(s) => s.clone(),
            Value::Binary(b) => String::from_utf8(b.clone()).unwrap(),
        }
    }
}

pub struct Settings {
    pub data_directory: String,
    pub app_name: String,
    pub defaults: HashMap<String, Value>,
    pub values: HashMap<String, Value>,
    pub modified: bool,
    pub has_errored: bool,
    pub should_auto_save: bool,
    pub loaded: bool,
    pub byte_stream: Vec<u8>,
    pub byte_input_stream: Vec<u8>,
    pub byte_output_stream: Vec<u8>,
    // pub executor: Thread,
    pub json: JsonValue,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            data_directory: "./data".to_string(),
            app_name: "app".to_string(),
            defaults: HashMap::new(),
            values: HashMap::new(),
            modified: false,
            has_errored: false,
            should_auto_save: true,
            loaded: false,
            byte_stream: Vec::new(),
            byte_input_stream: Vec::new(),
            byte_output_stream: Vec::new(),
            // executor: Thread::spawn(),
            json: object! {},
        }
    }

    pub fn load(&mut self) {
        // load keys
        self.load_values();
        //     OK => {}
        //     Err(e) => {
        //         self.has_errored = true;
        //         self.write_log(format!("Failed to load settings: {}", e)
        //     }
        // }
        // if loading failed, it still counts lmao
        self.loaded = true;
    }

    pub fn get_data_directory(&self) -> String {
        if self.data_directory == "" {
            get_app_data_directory(self.app_name.clone())
                .to_str()
                .unwrap()
                .to_string()
        } else {
            self.data_directory.clone()
        }
    }

    fn write_log(&self, message: String) {
        let log_file = self.get_data_directory() + "/settings.log";
        let mut file = match File::create(log_file.clone()) {
            Ok(file) => file,
            Err(e) => {
                // first check if the directory exists
                let dir = self.get_data_directory();
                debug!("Creating directory: {}", dir);
                match std::fs::create_dir(dir) {
                    Ok(_) => {}
                    Err(e) => {
                        panic!("Error creating directory: {}", e);
                    }
                }
                // try again
                match File::create(log_file) {
                    Ok(file) => file,
                    Err(e) => {
                        panic!("Error opening file: {}", e);
                    }
                }
            }
        };
        let msg = format!("[{}]: {}", get_current_time_string(), message);
        match file.write_all(msg.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                panic!("Error writing to file: {}", e);
            }
        };
    }

    pub fn set_data_directory(&mut self, data_directory: String) {
        self.data_directory = data_directory;
    }

    pub fn get_settings_file(&self) -> Fi {
        Fi::new_from_path_and_type(self.get_data_directory() + "/settings.bin", FType::Absolute)
    }

    pub fn get_backup_folder(&self) -> Fi {
        Fi::new_from_path(self.get_data_directory() + "/settings_backups")
    }

    pub fn get_backup_settings_file(&self) -> Fi {
        Fi::new_from_path_and_type(
            self.get_data_directory() + "/settings_backup.bin",
            FType::Absolute,
        )
    }

    pub fn load_values(&mut self) {
        if self.get_settings_file().exists() && !self.get_backup_settings_file().exists() {
            self.write_log(format!(
                "No Settings file found: {} and {}",
                self.get_settings_file().absolute_path(),
                self.get_backup_settings_file().absolute_path()
            ));
        }

        self.load_values_from_file(self.get_settings_file());
    }

    pub fn load_values_from_file(&mut self, file: Fi) {
        if !file.exists() {
            self.write_log(format!("No Settings file found: {}", file.absolute_path()));
            return;
        }

        let mut file = match File::open(file.absolute_path()) {
            Ok(file) => file,
            Err(e) => {
                self.write_log(format!("Failed to open settings file: {}", e));
                return;
            }
        };

        let mut buffer = Vec::new();
        match file.read_to_end(&mut buffer) {
            Ok(_) => {}
            Err(e) => {
                self.write_log(format!("Failed to read settings file: {}", e));
                return;
            }
        };

        self.byte_input_stream = buffer;
        self.read_values();
    }

    pub fn read_values(&mut self) {
        while self.byte_input_stream.len() > 0 {
            let key = self.read_string();
            let value = self.read_byte();
            match value {
                TYPE_BOOL => {
                    let b = self.read_bool();
                    self.values.insert(key.clone(), Value::Bool(b));
                }
                TYPE_INT => {
                    let i = self.read_int();
                    self.values.insert(key.clone(), Value::Int(i));
                }
                TYPE_LONG => {
                    let l = self.read_long();
                    self.values.insert(key.clone(), Value::Long(l));
                }
                TYPE_FLOAT => {
                    let f = self.read_float();
                    self.values.insert(key.clone(), Value::Float(f));
                }
                TYPE_STRING => {
                    let s = self.read_string();
                    self.values.insert(key.clone(), Value::String(s));
                }
                TYPE_BINARY => {
                    let b = self.read_binary();
                    self.values.insert(key.clone(), Value::Binary(b));
                }
                _ => {
                    let m = format!("Unknown type: {}", value);
                    self.write_log(m);
                }
            }
        }
    }

    pub fn read_byte(&mut self) -> u8 {
        let mut bytes = [0; 1];
        bytes.copy_from_slice(&self.byte_input_stream[0..1]);
        self.byte_input_stream = self.byte_input_stream[1..].to_vec();
        bytes[0]
    }

    pub fn read_bool(&mut self) -> bool {
        let mut bytes = [0; 1];
        bytes.copy_from_slice(&self.byte_input_stream[0..1]);
        self.byte_input_stream = self.byte_input_stream[1..].to_vec();
        bytes[0] == 1
    }

    pub fn read_int(&mut self) -> i32 {
        let mut bytes = [0; 4];
        bytes.copy_from_slice(&self.byte_input_stream[0..4]);
        self.byte_input_stream = self.byte_input_stream[4..].to_vec();
        i32::from_be_bytes(bytes)
    }

    pub fn read_long(&mut self) -> i64 {
        let mut bytes = [0; 8];
        bytes.copy_from_slice(&self.byte_input_stream[0..8]);
        self.byte_input_stream = self.byte_input_stream[8..].to_vec();
        i64::from_be_bytes(bytes)
    }

    pub fn read_float(&mut self) -> f32 {
        let mut bytes = [0; 4];
        bytes.copy_from_slice(&self.byte_input_stream[0..4]);
        self.byte_input_stream = self.byte_input_stream[4..].to_vec();
        f32::from_be_bytes(bytes)
    }

    pub fn read_string(&mut self) -> String {
        let mut bytes = [0; 4];
        bytes.copy_from_slice(&self.byte_input_stream[0..4]);
        self.byte_input_stream = self.byte_input_stream[4..].to_vec();
        let length = i32::from_be_bytes(bytes);
        let mut bytes = [0; 1];
        let mut string = String::new();
        for _ in 0..length {
            bytes.copy_from_slice(&self.byte_input_stream[0..1]);
            self.byte_input_stream = self.byte_input_stream[1..].to_vec();
            string.push(bytes[0] as char);
        }
        string
    }

    pub fn read_binary(&mut self) -> Vec<u8> {
        let mut bytes = [0; 4];
        bytes.copy_from_slice(&self.byte_input_stream[0..4]);
        self.byte_input_stream = self.byte_input_stream[4..].to_vec();
        let length = i32::from_be_bytes(bytes);
        let mut bytes = [0; 1];
        let mut binary = Vec::new();
        for _ in 0..length {
            bytes.copy_from_slice(&self.byte_input_stream[0..1]);
            self.byte_input_stream = self.byte_input_stream[1..].to_vec();
            binary.push(bytes[0]);
        }
        binary
    }

    pub fn save_values(&mut self) {
        self.byte_output_stream = Vec::new();
        for (key, value) in self.values.clone().iter() {
            self.write_string(key.clone());
            match value {
                Value::Bool(value) => {
                    self.write_byte(TYPE_BOOL);
                    self.write_bool(*value);
                }
                Value::Int(value) => {
                    self.write_byte(TYPE_INT);
                    self.write_int(*value);
                }
                Value::Long(value) => {
                    self.write_byte(TYPE_LONG);
                    self.write_long(*value);
                }
                Value::Float(value) => {
                    self.write_byte(TYPE_FLOAT);
                    self.write_float(*value);
                }
                Value::String(value) => {
                    self.write_byte(TYPE_STRING);
                    self.write_string(value.clone());
                }
                Value::Binary(value) => {
                    self.write_byte(TYPE_BINARY);
                    self.write_binary(value.clone());
                }
            }
        }

        self.write_log(format!("Write values: {}", self.byte_output_stream.len()));
        debug!("write values: {}", self.byte_output_stream.len());

        let mut file = File::create(self.get_settings_file().path).unwrap();
        file.write_all(&self.byte_output_stream).unwrap();
        debug!("data len: {:?}", self.byte_output_stream.len());

        self.write_log(format!(
            "Write file: {}",
            self.get_settings_file().absolute_path()
        ));

        // create backup
        // TODO: run this in a thread
        let mut backup_folder = self.get_backup_folder();
        debug!("Backup folder: {}", backup_folder.path);
        // create a new entry in the backup folder
        // first check if the folder exists
        if !backup_folder.is_directory() {
            debug!(
                "exists: {}, is_directory: {}",
                backup_folder.exists(),
                backup_folder.is_directory()
            );
            backup_folder.create_directory();
        }

        let mut files = backup_folder.list();
        // make sure first file is most recent, last is oldest
        files.sort_by(|mut a, b| b.last_modified().cmp(&a.last_modified()));

        Fi::new_from_file(file).copy_to(backup_folder.child(format!("{}.bin", millis())));

        // delete oldest backup if there are more than 10
        while files.len() > 10 {
            let mut file = files.pop().unwrap();
            file.delete();
        }
    }

    pub fn write_bool(&mut self, value: bool) {
        if value {
            self.write_byte(1);
        } else {
            self.write_byte(0);
        }
    }

    pub fn write_int(&mut self, value: i32) {
        let bytes = value.to_be_bytes();
        self.write_bytes(bytes.to_vec());
    }

    pub fn write_long(&mut self, value: i64) {
        let bytes = value.to_be_bytes();
        self.write_bytes(bytes.to_vec());
    }

    pub fn write_float(&mut self, value: f32) {
        let bytes = value.to_be_bytes();
        self.write_bytes(bytes.to_vec());
    }

    pub fn write_string(&mut self, value: String) {
        let length = value.len() as i32;
        self.write_int(length);
        for byte in value.bytes() {
            self.write_byte(byte);
        }
    }

    pub fn write_binary(&mut self, value: Vec<u8>) {
        let length = value.len() as i32;
        self.write_int(length);
        self.write_bytes(value);
    }

    pub fn write_byte(&mut self, value: u8) {
        self.byte_output_stream.push(value);
    }

    pub fn write_bytes(&mut self, value: Vec<u8>) {
        self.byte_output_stream.append(&mut value.clone());
    }

    pub fn default(&mut self, key: String, value: Value) {
        if !self.values.contains_key(&key) {
            self.values.insert(key, value);
        }
    }

    pub fn get_or_default(&mut self, key: String, value: Value) -> Value {
        if !self.values.contains_key(&key) {
            self.values.insert(key, value.clone());
            return value;
        }
        self.values.get(&key).unwrap().clone()
    }

    pub fn get(&mut self, key: String) -> Value {
        self.values.get(&key).unwrap().clone()
    }

    pub fn get_bool(&mut self, key: String) -> bool {
        match self.values.get(&key).unwrap() {
            Value::Bool(value) => *value,
            _ => panic!("Value is not a bool"),
        }
    }

    pub fn get_int(&mut self, key: String) -> i32 {
        match self.values.get(&key).unwrap() {
            Value::Int(value) => *value,
            _ => panic!("Value is not an int"),
        }
    }

    pub fn get_long(&mut self, key: String) -> i64 {
        match self.values.get(&key).unwrap() {
            Value::Long(value) => *value,
            _ => panic!("Value is not a long"),
        }
    }

    pub fn get_float(&mut self, key: String) -> f32 {
        match self.values.get(&key).unwrap() {
            Value::Float(value) => *value,
            _ => panic!("Value is not a float"),
        }
    }

    pub fn get_string(&mut self, key: String) -> String {
        match self.values.get(&key).unwrap() {
            Value::String(value) => value.clone(),
            _ => panic!("Value is not a string"),
        }
    }

    pub fn get_binary(&mut self, key: String) -> Vec<u8> {
        match self.values.get(&key).unwrap() {
            Value::Binary(value) => value.clone(),
            _ => panic!("Value is not a binary"),
        }
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.values.insert(key, value);
    }

    pub fn set_bool(&mut self, key: String, value: bool) {
        self.values.insert(key, Value::Bool(value));
    }

    pub fn set_int(&mut self, key: String, value: i32) {
        self.values.insert(key, Value::Int(value));
    }

    pub fn set_long(&mut self, key: String, value: i64) {
        self.values.insert(key, Value::Long(value));
    }

    pub fn set_float(&mut self, key: String, value: f32) {
        self.values.insert(key, Value::Float(value));
    }

    pub fn set_string(&mut self, key: String, value: String) {
        self.values.insert(key, Value::String(value));
    }

    pub fn set_binary(&mut self, key: String, value: Vec<u8>) {
        self.values.insert(key, Value::Binary(value));
    }

    pub fn remove(&mut self, key: String) {
        self.values.remove(&key);
    }

    pub fn contains(&mut self, key: String) -> bool {
        self.values.contains_key(&key)
    }

    pub fn get_keys(&mut self) -> Vec<String> {
        let mut keys = Vec::new();
        for key in self.values.keys() {
            keys.push(key.clone());
        }
        keys
    }

    pub fn get_values(&mut self) -> Vec<Value> {
        let mut values = Vec::new();
        for value in self.values.values() {
            values.push(value.clone());
        }
        values
    }

    pub fn get_entries(&mut self) -> Vec<(String, Value)> {
        let mut entries = Vec::new();
        for (key, value) in self.values.iter() {
            entries.push((key.clone(), value.clone()));
        }
        entries
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }

    pub fn size(&mut self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&mut self) -> bool {
        self.values.is_empty()
    }

    pub fn to_string(&mut self) -> String {
        let mut string = String::new();
        for (key, value) in self.values.iter() {
            string.push_str(&format!("{}: {}, ", key, value.to_string()));
        }
        string
    }

    pub fn force_save(&mut self) {
        // never loaded, nothing to save
        if !self.loaded {
            return;
        }
        self.save_values();
    }

    // todo: make json stuff
}

/// utility function to get settings
#[macro_export]
macro_rules! get_settings {
    ($data:expr) => {
        unsafe {
            $data = match SETTINGS {
                Some(ref mut s) => s,
                None => panic!("Settings not initialized"),
            }
        };
    }
}