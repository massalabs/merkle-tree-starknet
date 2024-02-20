extern crate static_assertions;
use anyhow::{anyhow, Result};

use std::{ffi::CStr, fs::File, io::Read, os::raw::c_char};
use strum_macros::{Display, EnumCount, EnumIter, EnumString, FromRepr};

mod yaml_parser;
use yaml_parser::parse_yaml;
mod test_generator;
use test_generator::generate_random_scenario;

#[repr(C)]
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumCount,
    EnumIter,
    FromRepr,
    EnumString,
    Display,
)]
pub enum CommandId {
    #[strum(serialize = "insert")]
    Insert,
    #[strum(serialize = "remove")]
    Remove,
    #[strum(serialize = "check_root_hash")]
    CheckRootHash,
    #[strum(serialize = "get")]
    Get,
    #[strum(serialize = "contains")]
    Contains,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Command {
    pub id: CommandId,
    pub arg1: Bytes,
    pub arg2: Bytes,
}

impl Command {
    pub fn new(id: CommandId, arg1: Bytes, arg2: Bytes) -> Self {
        Self { id, arg1, arg2 }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Bytes {
    pub ptr: *const u8,
    pub len: usize,
}

impl Bytes {
    pub fn new(v: Vec<u8>) -> Self {
        let boxed_vec: Box<[u8]> = v.into_boxed_slice();

        Self {
            len: boxed_vec.len(),
            ptr: Box::into_raw(boxed_vec) as *const u8,
        }
    }
}

impl From<&str> for Bytes {
    fn from(value: &str) -> Self {
        // println!("Converting string to Bytes: {} len {}", value, value.len());
        let mut v = value.as_bytes().to_vec();
        v.push(0);
        let v = v.into_boxed_slice();

        Self {
            len: v.len(),
            ptr: Box::into_raw(v) as *const u8,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CommandList {
    pub test_commands: *const Command,
    pub len: usize,
}

impl From<Vec<Command>> for CommandList {
    fn from(value: Vec<Command>) -> Self {
        let boxed_vec: Box<[Command]> = value.into_boxed_slice();
        Self {
            len: boxed_vec.len(),
            test_commands: Box::into_raw(boxed_vec) as *mut Command,
        }
    }
}

#[no_mangle]
pub extern "C" fn load_scenario(s: *const c_char) -> CommandList {
    let c_str = unsafe { CStr::from_ptr(s) };
    let file_path = c_str.to_string_lossy().into_owned();

    if !(file_path.ends_with(".yaml") || file_path.ends_with(".yml")) {
        panic!("ERROR: Test scenario file must be a yaml file (.yaml or .yml),\nprovided: {}", file_path);
    }
    read_yaml_file(&file_path).unwrap()
}

#[no_mangle]
pub extern "C" fn load_random() -> CommandList {
    generate_random_scenario()
}

#[no_mangle]
pub extern "C" fn free_scenario(cmd: CommandList) {
    if !cmd.test_commands.is_null() {
        // Convert the raw pointer back to a Vec,
        let vec = unsafe {
            Vec::from_raw_parts(
                cmd.test_commands as *mut Command,
                cmd.len,
                cmd.len,
            )
        };
        // Release the owned CStrings before deallocating the array
        // loop over the array to consume the CStrings
        for command in vec {
            unsafe {
                // Free array wrapper
                let _arg1 = Vec::from_raw_parts(
                    command.arg1.ptr as *mut Bytes,
                    command.arg1.len,
                    command.arg1.len,
                );

                let _arg2 = Vec::from_raw_parts(
                    command.arg2.ptr as *mut Bytes,
                    command.arg2.len,
                    command.arg2.len,
                );
            }
        }
    }
}

pub fn read_yaml_file(file_path: &str) -> Result<CommandList> {
    let mut file = File::open(file_path).map_err(|e| anyhow!("{}", e))?;
    let mut content = String::new();
    file.read_to_string(&mut content)
        .map_err(|e| anyhow!("{}", e))?;
    parse_yaml(&content)
        .map_err(|e| anyhow!("Error parsing {}: {}", file_path, e))
}
