const std = @import("std");

// Import the C functions from the `bindings.h` header file.
const c = @cImport({
    @cInclude("bindings.h");
});

extern fn add(a: u32, b: u32) callconv(.C) u32;

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();

    // Call the function `add`
    const result = c.add(35, 34);
    try stdout.print("Result from external addition of 35 and 34: {}\n", .{result});

    var s1: (*const [7:0]u8)  = "Hello, ";
    var s2 = "world!";
    const concatenated = c.concatenate_strings(&s1[0], &s2[0]);
    try stdout.print("{s}\n", .{concatenated});

    c.free_concatenated_string(concatenated);
}

