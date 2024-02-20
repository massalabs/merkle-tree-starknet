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
    counter: usize,

    pub fn insert(self: *TestEnv, key: *const Key, value: *const Value) anyerror!void {
        var gpa = std.heap.GeneralPurposeAllocator(.{}){};
        const allocator = gpa.allocator();
        defer _ = gpa.deinit();

        const key_str = try key_to_string(allocator, key);
        defer allocator.free(key_str);

        print("insert {any} {any}\n", .{ .{key_str}, .{value_to_string(value)} });
        self.counter += 1;
        print("counter: {}\n", .{self.counter});

        return;
    }

    pub fn remove(self: *TestEnv, key: *const Key) anyerror!void {
        var gpa = std.heap.GeneralPurposeAllocator(.{}){};
        const allocator = gpa.allocator();
        defer _ = gpa.deinit();

        const key_str = try key_to_string(allocator, key);
        defer allocator.free(key_str);

        print("remove {any}\n", .{key_str});
        self.counter += 1;
        print("counter: {}\n", .{self.counter});

        return;
    }

    pub fn get(self: *TestEnv, key: *const Key) anyerror!Value {
        var gpa = std.heap.GeneralPurposeAllocator(.{}){};
        const allocator = gpa.allocator();
        defer _ = gpa.deinit();

        const key_str = try key_to_string(allocator, key);
        defer allocator.free(key_str);

        print("get {any}\n", .{key_str});
        self.counter += 1;
        print("counter: {}\n", .{self.counter});

        return .{ .ptr = null, .len = 0 };
    }
    pub fn contains(self: *TestEnv, key: *const Key) anyerror!bool {
        var gpa = std.heap.GeneralPurposeAllocator(.{}){};
        const allocator = gpa.allocator();
        defer _ = gpa.deinit();

        const key_str = try key_to_string(allocator, key);
        defer allocator.free(key_str);

        print("contains {any}\n", .{key_str});
        self.counter += 1;
        print("counter: {}\n", .{self.counter});

        return false;
    }

    pub fn root_hash(self: *TestEnv) anyerror!Value {
        print("root_hash\n", .{});
        self.counter += 1;
        print("counter: {}\n", .{self.counter});

        return .{ .ptr = null, .len = 0 };
    }
};

fn create_test_env() TestEnv {
    print("create_test_env\n", .{});

    return .{
        .counter = 1,
    };
}

pub fn main() !void {
    print("start\n", .{});

    libtree.init_runner(TestEnv, create_test_env);
}
