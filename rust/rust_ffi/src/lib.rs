#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

use std::ffi::{CStr, CString};
use std::mem::ManuallyDrop;
use std::os::raw::{c_char, c_int};
use std::{array, ptr};

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

    // Leak the CString to ensure it lives long enough to be used from other languages
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

    // Leak the CString to ensure it lives long enough to be used from other languages
    c_string.into_raw()
}

static STRING: &str = "Hello World!\0";

#[no_mangle]
extern "C" fn ffi_string() -> *const u8 {
    STRING.as_ptr()
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct TestCommand {
    pub command: CommandId,
    pub arg1: *const c_char,
    pub arg2: *const c_char,
}
impl TestCommand {
    pub fn new(command: CommandId, arg1: &str, arg2: &str) -> Self {
        Self {
            command,
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
pub struct TestCommandList {
    pub test_commands: *const TestCommand,
    pub len: usize,
}
impl TestCommandList {
    pub fn new(commands: &[TestCommand]) -> Self {
        let mut commands: Vec<TestCommand> = commands.to_vec();

        commands.shrink_to_fit();
        // Leak the vector to ensure it lives long enough to be used from other languages
        let mut vec = ManuallyDrop::new(commands);

        let (ptr, len, _) = (vec.as_mut_ptr(), vec.len(), vec.capacity());

        Self {
            test_commands: ptr,
            len,
        }
    }
    pub fn default() -> Self {
        Self::new(&[])
    }
}

#[no_mangle]
pub extern "C" fn get_test2() -> TestCommandList {
    let cmd0: TestCommand = TestCommand::new(TC::Remove, "9", "8");
    let cmd1: TestCommand = TestCommand::new(TC::Insert, "1", "2");
    let cmd2: TestCommand = TestCommand::new(TC::Commit, "4", "5");

    TestCommandList::new(&[cmd0, cmd1, cmd2])
}

#[no_mangle]
pub extern "C" fn free_test(cmd: TestCommandList) {
    if !cmd.test_commands.is_null() {
        // Convert the raw pointer back to a Vec, 
        let vec = unsafe {
            Vec::from_raw_parts(
                cmd.test_commands as *mut TestCommand,
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
    End = 0,
    Insert = 1,
    Remove = 2,
    Commit = 3,
    RootHash = 4,
}
type TC = CommandId;

#[repr(C)]
#[derive(Debug)]
pub enum TestId {
    Test1,
    Test2,
    Test3,
}

#[repr(C)]
pub struct TestCases {
    pub test_cases: [TestId; 3],
}

#[no_mangle]
pub const extern "C" fn get_test_cases() -> TestCases {
    TestCases {
        test_cases: [TestId::Test1, TestId::Test2, TestId::Test3],
    }
}

#[no_mangle]
pub extern "C" fn get_test(id: TestId) -> TestCommandList {
    match id {
        TestId::Test1 => TestCommandList::default(),
        TestId::Test2 => get_test2(),
        TestId::Test3 => TestCommandList::default(),
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct VecCommands {
    pub commands: *mut TestCommand,
    pub len: usize,
}

#[no_mangle]
pub extern "C" fn leak() -> *mut VecCommands {
    let cmd0: TestCommand = TestCommand::new(CommandId::Remove, "9", "8");
    let cmd1: TestCommand = TestCommand::new(CommandId::Insert, "1", "2");
    let cmd2: TestCommand = TestCommand::new(CommandId::Commit, "4", "5");

    let vec = vec![cmd0, cmd1, cmd2];
    println!("vec: {:p}", &vec);
    let len = vec.len();
    let commands = Box::into_raw(Box::new(vec)) as *mut TestCommand;
    println!("commands: {:p}", commands);

    let ret = Box::into_raw(Box::new(VecCommands { len, commands }));
    println!("ret: {:p}", ret);
    ret
}

#[no_mangle]
pub extern "C" fn destroy_leak(s: *mut VecCommands) {
    unsafe {
        if !s.is_null() {
            let vec: Box<VecCommands> = Box::from_raw(s);
            println!("vec.commands: {:p}", vec.commands);

            let commands: Box<Vec<TestCommand>> =
                Box::from_raw(vec.commands as *mut Vec<TestCommand>);

            println!("commands: {:p}", commands);
            println!("commands: {:?}", commands);

            for command in commands.iter() {
                let _arg1 = CString::from_raw(command.arg1 as *mut i8);
                let _arg2 = CString::from_raw(command.arg2 as *mut i8);
            }
        }
    }
}
