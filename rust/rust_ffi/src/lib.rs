// #[macro_use]
extern crate static_assertions;
use yaml_rust::YamlLoader;

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn concatenate_strings(
    s1: *const c_char,
    s2: *const c_char,
) -> *mut c_char {
    // println!("s1: {:?}, s2: {:?}", s1, s2);

    // Convert C strings to Rust strings
    let c_str1 = unsafe { CStr::from_ptr(s1) };
    let c_str2 = unsafe { CStr::from_ptr(s2) };
    // println!("c_str1: {:?}, c_str2: {:?}", c_str1, c_str2);

    // Convert Rust strings to owned strings
    let rust_str1 = c_str1.to_string_lossy().into_owned();
    let rust_str2 = c_str2.to_string_lossy().into_owned();

    // Concatenate strings
    let result = rust_str1 + &rust_str2;

    // Convert the result back to a C string
    let c_string = CString::new(result).expect("Failed to create CString");
    // println!("c_string: {:?}", c_string);

    // Leak the CString to ensure it lives long enough to be used from other
    // languages
    c_string.into_raw()
}

#[no_mangle]
pub extern "C" fn free_concatenated_string(s: *mut c_char) {
    // Convert the C string back to a CString to free the memory
    unsafe {
        if s.is_null() {
            return;
        }
        let _ = CString::from_raw(s);
    }
}

#[no_mangle]
pub extern "C" fn print_string(s: *const c_char) {
    // Convert the C string to a Rust string
    let c_str = unsafe { CStr::from_ptr(s) };
    let rust_str = c_str.to_string_lossy().into_owned();

    // Print the Rust string
    println!("{}", rust_str);
}

#[no_mangle]
pub extern "C" fn get_string() -> *mut c_char {
    // Create a Rust string
    const RUST_STR: &str = "Hello from Rust!";

    // Convert the Rust string to a CString
    let c_string = CString::new(RUST_STR).expect("Failed to create CString");

    // Leak the CString to ensure it lives long enough to be used from other
    // languages
    c_string.into_raw()
}

static STRING: &str = "Hello World!\0";

#[no_mangle]
extern "C" fn ffi_string() -> *const u8 {
    STRING.as_ptr()
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Command {
    pub id: CommandId,
    pub arg1: *const c_char,
    pub arg2: *const c_char,
}

impl Command {
    pub fn new(id: CommandId, arg1: &str, arg2: &str) -> Self {
        // let arg1 = unsafe { std::str::from_utf8_unchecked(a) };
        Self {
            id,
            arg1: CString::new(arg1)
                .expect("Failed to create CString")
                .into_raw(),
            arg2: CString::new(arg2)
                .expect("Failed to create CString")
                .into_raw(),
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

    pub fn default() -> Self {
        Self::new(&[])
    }
}

#[no_mangle]
pub extern "C" fn get_test1() -> CommandList {
    let cmd_list = read_yaml_file("./scenario/1.yml").unwrap();

    dbg!(&cmd_list);
    cmd_list
}
#[no_mangle]
pub extern "C" fn get_test2() -> CommandList {
    let cmd0: Command = Command::new(TC::Remove, "0x490x500x51", "8");
    let cmd1: Command = Command::new(TC::Insert, "0x490x500x49", "2");
    let cmd2: Command = Command::new(TC::Commit, "0x490x500x49", "5");

    CommandList::new(&[cmd0, cmd1, cmd2])
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
            let _arg1 = unsafe { CString::from_raw(command.arg1 as *mut i8) };
            let _arg2 = unsafe { CString::from_raw(command.arg2 as *mut i8) };
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub enum CommandId {
    Insert,
    Remove,
    Commit,
    CheckRootHash,
}
type TC = CommandId;

#[repr(C)]
#[derive(Debug)]
pub enum TestId {
    Test1,
    Test2,
    Test3,
    Count,
}

#[repr(C)]
pub struct TestCases {
    pub test_cases: [TestId; TestId::Count as usize],
}

#[no_mangle]
pub const extern "C" fn get_test_cases() -> TestCases {
    TestCases {
        test_cases: [TestId::Test1, TestId::Test2, TestId::Test3],
    }
}

#[no_mangle]
pub extern "C" fn get_test(id: TestId) -> CommandList {
    match id {
        TestId::Test1 => get_test1(),
        TestId::Test2 => get_test2(),
        TestId::Test3 => CommandList::default(),
        TestId::Count => CommandList::default(),
    }
}

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
                    let b;

                    let arg1 = if let Some(key) = command["arg1"].as_vec() {
                        b = key
                            .into_iter()
                            .map(|x| x.as_i64().unwrap() as u8)
                            .collect::<Vec<_>>();
                        // let c = b.clone();

                        unsafe { std::str::from_utf8_unchecked(&b) }
                    } else {
                        command["arg1"].as_str().unwrap_or_else(|| "")
                    };

                    let arg2 = command["arg2"].as_str().unwrap_or_else(|| "");

                    let command = Command::new(TC::from(tc_type), arg1, arg2);

                    vec.push(command);
                }
            }
        }
    } else {
        panic!("No data found in YAML file");
    }

    Ok(CommandList::new(&vec))
}

impl From<&str> for TC {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "insert" => TC::Insert,
            "remove" => TC::Remove,
            "commit" => TC::Commit,
            "check_root_hash" => TC::CheckRootHash,
            _ => panic!("Unknown command type ! Allowed type : insert, remove, commit, check_root_hash, end")
        }
    }
}

// #[repr(C)]
// #[derive(Debug)]
// pub struct VecCommands {
//     pub commands: *mut Command,
//     pub len: usize,
// }

// #[no_mangle]
// pub extern "C" fn leak() -> *mut VecCommands {
//     let cmd0: Command = Command::new(CommandId::Remove, "9", "8");
//     let cmd1: Command = Command::new(CommandId::Insert, "1", "2");
//     let cmd2: Command = Command::new(CommandId::Commit, "4", "5");

//     let vec = vec![cmd0, cmd1, cmd2];
//     let len = vec.len();
//     let commands = Box::into_raw(Box::new(vec)) as *mut Command;
//     println!("commands: {:p}", commands);

//     VecCommands { len, commands }
// }

// #[no_mangle]
// pub extern "C" fn destroy_leak(s: VecCommands) {
//     unsafe {
//         // if !s.is_null() {
//         let vec: Box<VecCommands> = Box::from_raw(s);
//         println!("vec.commands: {:p}", vec.commands);

//         let commands: Box<Vec<Command>> =
//             Box::from_raw(vec.commands as *mut Vec<Command>);

//         let vec = Vec::from_raw_parts(s.commands, s.len, s.len);

//         println!("vec: {:?}", &vec);

//         for command in commands.iter() {
//             let _arg1 = CString::from_raw(command.arg1 as *mut i8);
//             let _arg2 = CString::from_raw(command.arg2 as *mut i8);
//         }
//         // }
//     }
// }
