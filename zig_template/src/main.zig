const std = @import("std");
const print = std.debug.print;
const mem = std.mem;

const libtree = @import("zig_common");
const Key = libtree.Key;
const Value = libtree.Value;
const key_to_string = libtree.key_to_string;
const value_to_string = libtree.value_to_string;

// must implement the interface
// pub const ITestTree = struct {
//     insert: *const fn (*ITestTree, *const Key, *const Value) anyerror!void,
//     remove: *const fn (*ITestTree, *const Key) anyerror!void,
//     get: *const fn (*ITestTree, *const Key) anyerror!Value,
//     contains: *const fn (*ITestTree, *const Key) anyerror!bool,
//     root_hash: *const fn (*ITestTree) anyerror!Value,
// };

const TestEnv = struct {
    // wrap your implementation here

    pub fn insert(self: *TestEnv, key: *const Key, value: *const Value) anyerror!void {
        return error.NotImplemented;
    }

    pub fn remove(self: *TestEnv, key: *const Key) anyerror!void {
        return error.NotImplemented;
    }

    pub fn get(self: *TestEnv, key: *const Key) anyerror!Value {
        return error.NotImplemented;
    }

    pub fn contains(self: *TestEnv, key: *const Key) anyerror!bool {
        return error.NotImplemented;
    }

    pub fn root_hash(self: *TestEnv) anyerror!Value {
        return error.NotImplemented;
    }
};

// implement a TestEnv factory
fn create_test_env() TestEnv {}

pub fn main() !void {
    libtree.init_runner(TestEnv, create_test_env);
}
