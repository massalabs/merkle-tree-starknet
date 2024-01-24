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
pub struct TestCommand2 {
    pub command: TestCommand,
    pub arg1: *const c_char,
    pub arg2: *const c_char,
}

#[repr(C)]
#[derive(Debug)]
pub struct TestCommandList2 {
    pub test_commands: *const TestCommand2,
    pub len: usize,
}

#[no_mangle]
pub extern "C" fn get_test2() -> TestCommandList2 {
    let cmd1: TestCommand2 = TestCommand2 {
        command: TC::Insert,
        arg1: CString::new("1")
            .expect("Failed to create CString")
            .into_raw(),
        arg2: CString::new("2")
            .expect("Failed to create CString")
            .into_raw(),
    };

    let cmd2: TestCommand2 = TestCommand2 {
        command: TestCommand::Commit,
        arg1: CString::new("4")
            .expect("Failed to create CString")
            .into_raw(),
        arg2: CString::new("5")
            .expect("Failed to create CString")
            .into_raw(),
    };

    let mut vec = vec![cmd1, cmd2];
    // make sure len and capacity are the same
    vec.shrink_to_fit();
    // Leak the vector to ensure it lives long enough to be used from other languages
    let mut vec = ManuallyDrop::new(vec);

    let (ptr, len, _) = (vec.as_mut_ptr(), vec.len(), vec.capacity());

    TestCommandList2 {
        test_commands: ptr,
        len,
    }
}

#[no_mangle]
pub extern "C" fn free_test(cmd: TestCommandList2) {
    if !cmd.test_commands.is_null() {
        // Convert the raw pointer back to a Vec
        let vec = unsafe {
            Vec::from_raw_parts(
                cmd.test_commands as *mut TestCommand2,
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
pub enum TestCommand {
    End,
    Insert,
    Remove,
    Commit,
    RootHash,
}
type TC = TestCommand;

// Test scenarios
const TEST1: [TestCommand; 4] = [TC::Insert, TC::Commit, TC::Remove, TC::End];
const TEST2: [TestCommand; 5] =
    [TC::Insert, TC::Commit, TC::Remove, TC::RootHash, TC::End];
const TEST3: [TestCommand; 3] = [TC::Insert, TC::Commit, TC::End];

const TEST_CASES: [&[TestCommand]; 3] = [&TEST1, &TEST2, &TEST3];

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

#[repr(C)]
pub struct TestCommandList {
    pub test_commands: *const TestCommand,
    pub len: usize,
}

#[no_mangle]
pub extern "C" fn get_test(id: TestId) -> TestCommandList {
    match id {
        TestId::Test1 => TestCommandList {
            test_commands: TEST1.as_ptr(),
            len: TEST1.len(),
        },
        TestId::Test2 => TestCommandList {
            test_commands: TEST2.as_ptr(),
            len: TEST2.len(),
        },
        TestId::Test3 => TestCommandList {
            test_commands: TEST3.as_ptr(),
            len: TEST3.len(),
        },
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct VecCommands {
    pub commands: *mut TestCommand2,
    pub len: usize,
}

#[no_mangle]
pub extern "C" fn leak() -> *mut VecCommands {
    let cmd0: TestCommand2 = TestCommand2{
        command: TestCommand::Remove,
        arg1: CString::new("9")
            .expect("Failed to create CString")
            .into_raw(),
        arg2: CString::new("8")
            .expect("Failed to create CString")
            .into_raw(),
    };
    let cmd1: TestCommand2 = TestCommand2 {
        command: TestCommand::Insert,
        arg1: CString::new("1")
            .expect("Failed to create CString")
            .into_raw(),
        arg2: CString::new("2")
            .expect("Failed to create CString")
            .into_raw(),
    };

    let cmd2: TestCommand2 = TestCommand2 {
        command: TestCommand::Commit,
        arg1: CString::new("4")
            .expect("Failed to create CString")
            .into_raw(),
        arg2: CString::new("5")
            .expect("Failed to create CString")
            .into_raw(),
    };

    let vec = vec![cmd0, cmd1, cmd2];

    Box::into_raw(Box::new(VecCommands {
        len: vec.len(),
        commands:Box::into_raw(Box::new(vec)) as *mut TestCommand2,
    }))
}

#[no_mangle]
pub extern "C" fn destroy_leak(s: *mut VecCommands) {
    unsafe {
        if !s.is_null() {
            let vec: Box<VecCommands> = Box::from_raw(s);
            println!("vec.commands: {:p}", vec.commands);

            let commands: Box<Vec<TestCommand2>> = Box::from_raw(vec.commands as *mut Vec<TestCommand2>);

            println!("commands: {:p}", commands);
            println!("commands: {:?}", commands);

            for command in commands.iter() {
                let _arg1 = CString::from_raw(command.arg1 as *mut i8);
                let _arg2 = CString::from_raw(command.arg2 as *mut i8);
            }
        }
    }
}

/* type TestId = u64;
#[repr(C)]
struct TestCases<const COUNT: usize> {
    test_cases: [TestId; COUNT],
}
 */
/* #[repr(C)]
struct TestCase<const COUNT: usize> {
    test_steps: [&'static str; COUNT],
    test_id: TestId,
}

#[no_mangle]
const extern "C" fn get_test_cases() -> TestCases<5> {
    let test_cases: [u64; 5] = [1, 2, 3, 4, 5];
    TestCases { test_cases }
}
 */
/* #[no_mangle]
extern "C" fn get_test(id: TestId) -> TestCase<3> {
    // let test_steps = ["step1", "step2", "step3"];
    // let mut t = Vec::with_capacity(c);
    // t.push("step1");
    // t.push("step2");
    // t.push("step3");
    // // let test_steps = t.as_slice();

    let my_str_array: Vec<&str> =
        ["step1", "step2", "step3"].iter().map(|s| *s).collect();

    let test_steps: [&str; 3] = my_str_array.try_into().unwrap_or_else(|v| {
        panic!("Expected an array of length {}, but got {:?}", 3, v)
    });

    TestCase {
        test_steps,
        test_id: id,
    }
}
 */
/* pub mod TestModule {

    trait SliceContainer {
        type Item;

        extern "C" fn as_slice(&self) -> &[Self::Item];
    }

    pub struct GenericSliceContainer<T: 'static> {
        pub data: Vec<T>,
    }

    impl<T: 'static> SliceContainer for GenericSliceContainer<T> {
        type Item = T;

        #[no_mangle]
        extern "C" fn as_slice(&self) -> &[T] {
            &self.data
        }
    }
}
 */
