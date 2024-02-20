const clap = @import("clap");
const std = @import("std");
const testing = std.testing;
const debug = std.debug;
const print = debug.print;

const mem = std.mem;

const io = std.io;

const c = @cImport({
    @cInclude("bindings.h");
});

const cString = @cImport({
    @cInclude("string.h");
});

pub const Key = c.Bytes;
pub const Value = c.Bytes;

const ScenerioError = error{ InsertErr, RemoveErr, GetErr, ContainsErr, RootHashErr };

pub fn init_runner(comptime T: type, creator: fn () T) void {
    print("init_runner\n", .{});

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const params = comptime clap.parseParamsComptime(
        \\-h, --help             Display this help and exit.
        \\-f, --file <str>       Test scenario file.
        \\-d, --dir <str>        Test scenario directory.
        \\
    );

    var diag = clap.Diagnostic{};
    var res = clap.parse(clap.Help, &params, clap.parsers.default, .{
        .diagnostic = &diag,
        .allocator = gpa.allocator(),
    }) catch |err| {
        // Report useful error and exit
        diag.report(io.getStdErr().writer(), err) catch {};

        print("Error: {}\n", .{err});
        return;
    };
    defer res.deinit();

    if ((res.args.help != 0) or (res.args.file == null and res.args.dir == null)) {
        const m = clap.help(std.io.getStdErr().writer(), clap.Help, &params, .{});

        print("{any}", .{m});
        clap.help(std.io.getStdErr().writer(), clap.Help, &params, .{}) catch {};
        return;
    }

    if (res.args.file != null and res.args.dir != null) {
        print("Error: --file and --dir are mutually exclusive\n", .{});
        return;
    }

    if (res.args.file) |f| {
        if (!is_yml_file(f)) {
            print("Error: file must have .yml extension\n", .{});
            return;
        }
        const c_str = gpa.allocator().dupeZ(u8, f) catch |err| {
            print("Error: {}\n", .{err});
            return;
        };
        defer gpa.allocator().free(c_str);

        start_test(c_str, std.fs.path.basename(f), T, creator) catch |err| {
            print("Error: {}\n", .{err});
        };
        return;
    }

    if (res.args.dir) |d| {
        const dir = std.fs.cwd().openDir(d, .{ .iterate = true }) catch |err| {
            print("Error: {}\n", .{err});
            return;
        };
        var dir_iter = dir.walk(gpa.allocator()) catch |err| {
            print("Error: {}\n", .{err});
            return;
        };
        defer dir_iter.deinit();

        while (dir_iter.next() catch |err| {
            print("Error: {}\n", .{err});
            return;
        }) |entry| {
            if (is_yml_file(entry.basename)) {
                print("path: {s}\n", .{entry.basename});
                const f_name = std.fs.path.joinZ(gpa.allocator(), &.{ d, entry.basename }) catch |err| {
                    print("Error: {}\n", .{err});
                    return;
                };
                defer gpa.allocator().free(f_name);

                start_test(f_name, entry.basename, T, creator) catch |err| {
                    print("Error: {}\n", .{err});
                };
            }
        }
    }
}

pub fn key_to_string(allocator: mem.Allocator, key: *const Key) ![]u8 {
    var str = try allocator.alloc(u8, 4 * key.len + 3);

    const ret = str;
    str[0] = '[';
    str.ptr += 1;

    const keys = key.ptr[0..key.len];
    for (keys) |k| {
        str = try std.fmt.bufPrint(str, "{d},", .{k});
    }

    // overwrite last comma
    str.ptr -= 1;
    str[0] = ']';
    str[1] = 0;

    return ret;
}

pub fn value_to_string(value: *const Value) *const u8 {
    return @ptrCast(@alignCast(value.*.ptr));
}

fn is_yml_file(file: []const u8) bool {
    return mem.eql(u8, file[file.len - 4 .. file.len], ".yml");
}

fn start_test(path: [:0]const u8, name: []const u8, comptime T: type, creator: fn () T) !void {
    print("start_test: {s}\n", .{path});
    print("name: {s}\n", .{name});

    var test_env: T = creator();

    const command_list = c.load_scenario(path);
    defer c.free_scenario(command_list);

    try run_test(command_list, T, &test_env);
}

fn run_test(command_list: c.CommandList, comptime T: type, test_env: *T) !void {
    const commands = command_list.test_commands[0..command_list.len];

    for (commands) |cmd| {
        try run_command(cmd, comptime T, test_env);
    }

    return;
}

fn run_command(command: c.Command, comptime T: type, test_env: *T) !void {
    switch (command.id) {
        c.Insert => {
            const key = command.arg1;
            const value = command.arg2;

            try test_env.insert(&key, &value);
        },

        c.Remove => {
            const key = command.arg1;

            try test_env.remove(&key);
        },

        c.Get => {
            const key = command.arg1;
            const value = command.arg2;
            const value_ptr: []const u8 = value.ptr[0..value.len];

            const get = try test_env.get(&key);

            if (get.ptr == null) {
                return error.GetErr;
            }

            const get_ptr: []const u8 = get.ptr[0..get.len];

            if (value.len != get.len or !mem.eql(u8, value_ptr, get_ptr)) {
                return error.GetErr;
            }
        },

        c.Contains => {
            const key = command.arg1;
            const value = command.arg2;
            const value_ptr: []const u8 = value.ptr[0..value.len];

            const ref_value = std.mem.eql(u8, value_ptr, "true");

            const contains = try test_env.contains(&key);

            if (ref_value != contains) {
                return error.ContainsErr;
            }
        },

        c.CheckRootHash => {
            const value = command.arg1;
            const value_ptr: []const u8 = value.ptr[0..value.len];

            const root_hash = try test_env.root_hash();

            if (root_hash.ptr == null) {
                return error.RootHashErr;
            }

            const root_hash_ptr: []const u8 = root_hash.ptr[0..root_hash.len];

            if (value.len != root_hash.len or !mem.eql(u8, value_ptr, root_hash_ptr)) {
                return error.RootHashErr;
            }
        },

        else => {
            print("Error: unknown command id: {}\n", .{command.id});
            return error.Unreachable;
        },
    }
    return;
}
