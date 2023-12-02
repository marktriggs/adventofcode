// (cd /mnt/ssd/mst-home/projects/adventofcode/2023/src && zig run main.zig)

const std = @import("std");

pub fn main() !void {
    // try day1Pt1();
    // try day1Pt2();

    try day2Pt1();
    try day2Pt2();
}

const Sample = struct {
    red: u64,
    green: u64,
    blue: u64,
};

pub fn day2Pt1() !void {
    var file = try std.fs.cwd().openFile("input_files/day2.txt", .{ .mode = std.fs.File.OpenMode.read_only });

    var reader = file.reader();
    var buf: [1024]u8 = undefined;

    var result: usize = 0;

    while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var game_it = std.mem.splitSequence(u8, line, ":");

        var game = try std.fmt.parseUnsigned(usize, game_it.next().?[5..], 10);
        var samples = std.mem.trim(u8, game_it.next().?, " ");

        var sample_it = std.mem.splitSequence(u8, samples, "; ");

        var game_possible = true;

        while (sample_it.next()) |sample_str| {
            var cube_it = std.mem.splitSequence(u8, sample_str, ", ");

            var sample = Sample { .red = 0, .green = 0, .blue = 0 };

            while (cube_it.next()) |cube| {
                var cube_bits = std.mem.splitSequence(u8, cube, " ");
                var count = try std.fmt.parseUnsigned(usize, cube_bits.next().?, 10);
                var colour = cube_bits.next().?;

                switch (colour[0]) {
                    'r' => { sample.red = count; },
                    'g' => { sample.green = count; },
                    'b' => { sample.blue = count; },
                    else => { unreachable; }
                }
            }

            if (sample.red > 12 or sample.green > 13 or sample.blue > 14) {
                game_possible = false;
            }
        }

        if (game_possible) {
            result += game;
        }
    }

    std.debug.print("Sum of possible games: {}\n", .{result});
}


pub fn day2Pt2() !void {
    var file = try std.fs.cwd().openFile("input_files/day2.txt", .{ .mode = std.fs.File.OpenMode.read_only });

    var reader = file.reader();
    var buf: [1024]u8 = undefined;

    var result: usize = 0;

    while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var game_it = std.mem.splitSequence(u8, line, ":");

        _ = game_it.next();
        var samples = std.mem.trim(u8, game_it.next().?, " ");

        var min_sample = Sample { .red = 0, .green = 0, .blue = 0 };

        var sample_it = std.mem.splitSequence(u8, samples, "; ");
        while (sample_it.next()) |sample_str| {
            var cube_it = std.mem.splitSequence(u8, sample_str, ", ");

            var sample = Sample { .red = 0, .green = 0, .blue = 0 };

            while (cube_it.next()) |cube| {
                var cube_bits = std.mem.splitSequence(u8, cube, " ");
                var count = try std.fmt.parseUnsigned(usize, cube_bits.next().?, 10);
                var colour = cube_bits.next().?;

                switch (colour[0]) {
                    'r' => { sample.red = count; },
                    'g' => { sample.green = count; },
                    'b' => { sample.blue = count; },
                    else => { unreachable; }
                }
            }

            min_sample.red = @max(min_sample.red, sample.red);
            min_sample.green = @max(min_sample.green, sample.green);
            min_sample.blue = @max(min_sample.blue, sample.blue);
        }

        result += (min_sample.red * min_sample.green * min_sample.blue);
    }

    std.debug.print("Sum of power sets: {d}\n", .{result});
}


pub fn day1Pt1() !void {
    var file = try std.fs.cwd().openFile("input_files/day1.txt", .{ .mode = std.fs.File.OpenMode.read_only });

    var reader = file.reader();
    var buf: [1024]u8 = undefined;

    var sum: usize = 0;

    while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var digits: [2]u8 = .{0, 0};
        var offset: usize = 0;

        for (line) |ch| {
            if (std.ascii.isDigit(ch)) {
                digits[offset] = ch - '0';

                if (offset == 0) {
                    offset += 1;
                    digits[offset] = digits[offset - 1];
                }
            }
        }

        // std.debug.print("Line: {s}\n", .{line});
        // std.debug.print("Digits: {d}\n", .{digits});

        sum += (digits[0] * 10) + digits[1];
    }

    std.debug.print("Sum (pt1): {}\n", .{sum});
}

pub fn day1Pt2() !void {
    var file = try std.fs.cwd().openFile("input_files/day1.txt", .{ .mode = std.fs.File.OpenMode.read_only });

    var reader = file.reader();
    var buf: [1024]u8 = undefined;

    var sum: usize = 0;

    var words = [_][]const u8 {"one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};

    while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var digits: [2]u8 = .{0, 0};
        var offset: usize = 0;

        var i: usize = 0;
        while (i < line.len) {
            var value: ?u8 = null;

            if (std.ascii.isDigit(line[i])) {
                value = line[i] - '0';
            }

            if (value == null) {
                for (0.., words) |word_idx, word| {
                    if (std.mem.startsWith(u8, line[i..], word)) {
                        value = @intCast(word_idx + 1);
                        break;
                    }
                }
            }

            if (value != null) {
                digits[offset] = value.?;

                if (offset == 0) {
                    offset += 1;
                    digits[offset] = value.?;
                }
            }

            i += 1;
        }

        // std.debug.print("Line: {s} -> {d}\n", .{line, digits});

        sum += (digits[0] * 10) + digits[1];

        // std.debug.print("+= {d} = {d}\n", .{(digits[0] * 10) + digits[1], sum});
    }

    std.debug.print("Sum (pt2): {}", .{sum});
}

pub fn example_code() !void {
        // var line_buf = try allocator.dupe(u8, line);
        // var replaced = try allocator.dupe(u8, line);
        // _ = std.mem.replace(u8, line_buf, "one", "1", replaced); line_buf = replaced;
        // _ = std.mem.replace(u8, line_buf, "two", "2", replaced); line_buf = replaced;
        // _ = std.mem.replace(u8, line_buf, "three", "3", replaced); line_buf = replaced;
        // _ = std.mem.replace(u8, line_buf, "four", "4", replaced); line_buf = replaced;
        // _ = std.mem.replace(u8, line_buf, "five", "5", replaced); line_buf = replaced;
        // _ = std.mem.replace(u8, line_buf, "six", "6", replaced); line_buf = replaced;
        // _ = std.mem.replace(u8, line_buf, "seven", "7", replaced); line_buf = replaced;
        // _ = std.mem.replace(u8, line_buf, "eight", "8", replaced); line_buf = replaced;
        // _ = std.mem.replace(u8, line_buf, "nine", "9", replaced); line_buf = replaced;


    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    var allocator = arena.allocator();

    var stuff = try std.ArrayList(u64).initCapacity(allocator, 1024);

    try stuff.append(123);
    try stuff.append(456);
    try stuff.append(789);

    std.debug.print("Size of stuff: {d}\n", .{
        stuff.items.len
    });

    std.debug.print("moo: {d}\n", .{stuff.items[1]});

    var table = std.StringHashMap(u64).init(allocator);

    try table.put("hello world", 666);

    std.debug.print("Table size: {d}\n", .{table.count()});

    try table.put("goodbye world", 123);

    std.debug.print("Table size: {d}\n", .{table.count()});

    var it = table.iterator();
    while (it.next()) |entry| {
        const key = entry.key_ptr;
        const value = entry.value_ptr;

        std.debug.print("{s}, {d}\n", .{key.*, value.*});
    }

    var initialised_array = try std.ArrayList(u64).initCapacity(allocator, 128);
    try initialised_array.appendSlice(&std.mem.zeroes([128]u64));
    initialised_array.items[5] = 666;

    std.debug.print("{}\n", .{initialised_array});

    var file = try std.fs.openFileAbsolute("/etc/group", .{ .mode = std.fs.File.OpenMode.read_only });

    var bytes = try file.readToEndAlloc(allocator, std.math.maxInt(usize));
    var it2 = std.mem.split(u8, bytes, "\n");
    while (it2.next()) |line| {
        std.debug.print("LINE: {s}\n", .{line});
    }

    try file.seekTo(0);
    var reader = file.reader();

    var buffer = try std.ArrayList(u8).initCapacity(allocator, 4096);
    try buffer.appendSlice(&std.mem.zeroes([4096]u8));
    var len = try reader.read(buffer.items);

    std.debug.print("{s}", .{buffer.items[0..len]});

    var s = "12345";
    var parsed = try std.fmt.parseUnsigned(u64, s, 10);

    std.debug.print("Parsed value: {d}\n", . {parsed});
}
