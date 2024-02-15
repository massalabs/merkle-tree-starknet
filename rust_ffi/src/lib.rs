// #[macro_use]
extern crate static_assertions;
use yaml_rust::YamlLoader;


use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::os::raw::c_char;

#[repr(C)]
#[derive(Debug, Clone)]
pub enum CommandId {
    Insert,
    Remove,
    CheckRootHash,
    Get,
    Contains,
}
type TC = CommandId;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Command {
    pub id: CommandId,
    pub arg1: ArrayWrapper,
    pub arg2: *const c_char,
}

impl Command {
    pub fn new(id: CommandId, arg1: ArrayWrapper, arg2: &str) -> Self {
        Self {
            id,
            arg1,
            arg2: CString::new(arg2)
                .expect("Failed to create CString")
                .into_raw(),
        }
    }
}


#[repr(C)]
#[derive(Debug, Clone)]
pub struct ArrayWrapper {
    pub ptr: *const u8,
    pub len: usize,
}

impl ArrayWrapper {
    pub fn new(v: Vec<u8>) -> Self {
        let boxed_vec: Box<[u8]> = v.into_boxed_slice();
        Self {
            len: boxed_vec.len(),
            ptr: Box::into_raw(boxed_vec) as *const u8,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct CommandList {
    pub test_commands: *const Command,
    pub len: usize,
}

impl CommandList {
    pub fn new(commands: &[Command]) -> Self {
        let commands: Vec<Command> = commands.to_vec();
        let boxed_vec: Box<[Command]> = commands.into_boxed_slice();

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
    let cmd_list = read_yaml_file(&file_path).unwrap();
    // dbg!(&cmd_list);
    cmd_list
}

#[no_mangle]
pub extern "C" fn free_test(cmd: CommandList) {
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
                let _array_wrapper =
                Vec::from_raw_parts(
                    command.arg1.ptr as *mut ArrayWrapper,
                    command.arg1.len,
                    command.arg1.len,
                );

                let _arg2 =  CString::from_raw(command.arg2 as *mut i8);
            }
        }
    }
}


//TODO
// check that id read from yaml file are monotonic and increasing

pub fn read_yaml_file(file_path: &str) -> std::io::Result<CommandList> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    let docs = YamlLoader::load_from_str(&content).expect("Error parsing YAML");

    let mut vec = vec![];

    if let Some(doc) = docs.get(0) {
        if let Some(commands) = doc["commands"].as_vec() {
            for command in commands {
                if let Some(tc_type) = command["action"].as_str() {
                    // if arg1 is an array, convert it to a Vec<u8>
                    let arg1 = if let Some(key) = command["arg1"].as_vec() {
                        key
                            .into_iter()
                            .map(|x| x.as_i64().unwrap() as u8)
                            .collect::<Vec<_>>()
                    } else {
                        // if arg1 is a string, convert it to a Vec<u8>
                        command["arg1"].as_str().to_owned().unwrap().as_bytes().to_vec()
                    };

                    let arg2 = command["arg2"].as_str().unwrap_or_else(|| "");
                    let command = Command::new(TC::from(tc_type), ArrayWrapper::new(arg1), arg2);

                    vec.push(command);
                }
            }
        }
    } else {
        panic!("No data found in YAML file");
    }

    Ok(CommandList::new(&vec))
}

/// Convert a string action to a Command
impl From<&str> for TC {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "insert" => TC::Insert,
            "remove" => TC::Remove,
            "check_root_hash" => TC::CheckRootHash,
            "contains" => TC::Contains,
            "get" => TC::Get,
            _ => panic!("Unknown command type: {}\n", value),
        }
    }
}