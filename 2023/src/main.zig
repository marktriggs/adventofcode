// (cd /mnt/ssd/mst-home/projects/adventofcode/2023/src && zig build-exe -O ReleaseFast main.zig && ./main)

const std = @import("std");

pub fn main() !void {
    // try day1Pt1();
    // try day1Pt2();

    // try day2Pt1();
    // try day2Pt2();

    // try day3Pt1();
    // try day3Pt2();

    // try day4Pt1();
    // try day4Pt2();

    // try day5Pt1();
    // try day5Pt2();

    // try day6Pt1();
    try day6Pt2();
}

const RaceResult = struct {
    race_time_ms: usize,
    winning_distance_mm: usize,
};

pub fn day6Pt1() !void {
    // var races = [_]RaceResult {
    //     RaceResult { .race_time_ms = 7, .winning_distance_mm = 9 },
    //     RaceResult { .race_time_ms = 15, .winning_distance_mm = 40 },
    //     RaceResult { .race_time_ms = 30, .winning_distance_mm = 200 },
    // };

    var races = [_]RaceResult {
        RaceResult { .race_time_ms = 41, .winning_distance_mm = 214 },
        RaceResult { .race_time_ms = 96, .winning_distance_mm = 1789 },
        RaceResult { .race_time_ms = 88, .winning_distance_mm = 1127 },
        RaceResult { .race_time_ms = 94, .winning_distance_mm = 1055 },
    };

    var result: usize = 1;

    for (races) |race| {
        var win_count: usize = 0;

        var hold_ms: usize = 1;
        while (hold_ms < race.race_time_ms): (hold_ms += 1) {
            var distance_travelled_mm = (race.race_time_ms - hold_ms) * hold_ms;

            if (distance_travelled_mm > race.winning_distance_mm) {
                // std.debug.print("Can win race by holding for {d} ms (travelled {d} mm)\n", .{hold_ms, distance_travelled_mm});
                win_count += 1;
            }
        }

        result *= win_count;
    }

    std.debug.print("Part 1 result: {}\n", .{result});
}

pub fn day6Pt2() !void {
    // 71503
    // var race = RaceResult { .race_time_ms = 71530, .winning_distance_mm = 940200 };

    var race = RaceResult { .race_time_ms = 41968894, .winning_distance_mm = 214178911271055 };

    var win_count: usize = 0;

    var hold_ms: usize = 1;
    while (hold_ms < race.race_time_ms): (hold_ms += 1) {
        var distance_travelled_mm = (race.race_time_ms - hold_ms) * hold_ms;

        if (distance_travelled_mm > race.winning_distance_mm) {
            // std.debug.print("Can win race by holding for {d} ms (travelled {d} mm)\n", .{hold_ms, distance_travelled_mm});
            win_count += 1;
        }
    }

    std.debug.print("Part 2 win count: {}\n", .{win_count});
}



const MapRange = struct {
    dst_start: usize,
    src_start: usize,
    src_end: usize,
    len: usize,
};

const Map = struct {
    ranges: []MapRange,

    fn new(allocator: std.mem.Allocator, ranges: *[]MapRange) !Map {
        var sorted_ranges = try allocator.dupe(MapRange, ranges.*);

        std.sort.heap(MapRange, sorted_ranges, {}, lessThanSrcStart);

        return Map {
            .ranges = sorted_ranges,
        };
    }

    fn searchRanges(context: void, src_value: usize, range: MapRange) std.math.Order {
        _ = context;

        if (src_value < range.src_start) {
            return .lt;
        } else if (src_value >= range.src_end) {
            return .gt;
        } else {
            return .eq;
        }
    }

    fn lessThanSrcStart(context: void, a: MapRange, b: MapRange) bool {
        _ = context;
        return a.src_start < b.src_start;
    }

    fn map(self: *const Map, src_value: usize) usize {
        var idx = std.sort.binarySearch(MapRange,
                              src_value,
                              self.ranges,
                              {},
                              searchRanges);

        if (idx == null) {
            return src_value;
        } else {
            var range = self.ranges[idx.?];
            return range.dst_start + (src_value - range.src_start);
        }

        return src_value;
    }
};

pub fn day5Pt1() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    var allocator = arena.allocator();

    var buf: [1024]u8 = undefined;
    var file = try std.fs.cwd().openFile("input_files/day5.txt", .{ .mode = std.fs.File.OpenMode.read_only });
    var reader = file.reader();

    var seeds = blk: {
        var next_line = (try reader.readUntilDelimiterOrEof(&buf, '\n')).?;
        var tokens = std.mem.splitAny(u8, next_line, ": ");
        _ = tokens.next();       // label
        var seeds = std.ArrayList(usize).init(allocator);

        while (tokens.next()) |seed| {
            if (seed.len > 0) {
                try seeds.append(try std.fmt.parseUnsigned(usize, seed, 10));
            }
        }

        break :blk seeds;
    };

    var mappings = std.StringHashMap(Map).init(allocator);

    // Skip empty line
    try reader.skipUntilDelimiterOrEof('\n');

    while (true) {
        var label = try allocator.dupe(u8, reader.readUntilDelimiter(&buf, ' ') catch break);
        try reader.skipUntilDelimiterOrEof('\n');

        // Read one or more maps
        var ranges = std.ArrayList(MapRange).init(allocator);

        while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
            if (line.len == 0) {
                break;
            }

            var it = std.mem.splitAny(u8, line, " ");

            var dst_start = try std.fmt.parseUnsigned(usize, it.next().?, 10);
            var src_start = try std.fmt.parseUnsigned(usize, it.next().?, 10);
            var len = try std.fmt.parseUnsigned(usize, it.next().?, 10);

            try ranges.append(MapRange {
                .dst_start = dst_start,
                .src_start = src_start,
                .src_end = src_start + len,
                .len = len,
            });
        }

        try mappings.put(label, try Map.new(allocator, &ranges.items));
    }

    var lowest_location: usize = std.math.maxInt(usize);

    for (seeds.items) |seed| {
        var mapped: usize = seed;

        mapped = mappings.getPtr("seed-to-soil").?.map(mapped);
        mapped = mappings.getPtr("soil-to-fertilizer").?.map(mapped);
        mapped = mappings.getPtr("fertilizer-to-water").?.map(mapped);
        mapped = mappings.getPtr("water-to-light").?.map(mapped);
        mapped = mappings.getPtr("light-to-temperature").?.map(mapped);
        mapped = mappings.getPtr("temperature-to-humidity").?.map(mapped);
        mapped = mappings.getPtr("humidity-to-location").?.map(mapped);

        if (mapped < lowest_location) {
            lowest_location = mapped;
        }
    }

    std.debug.print("Part 1 lowest location was {d}\n", . {
        lowest_location
    });
}


pub fn day5Pt2() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    var allocator = arena.allocator();

    var buf: [1024]u8 = undefined;
    var file = try std.fs.cwd().openFile("input_files/day5.txt", .{ .mode = std.fs.File.OpenMode.read_only });
    var reader = file.reader();

    var seed_ranges = blk: {
        var next_line = (try reader.readUntilDelimiterOrEof(&buf, '\n')).?;
        var tokens = std.mem.splitAny(u8, next_line, ": ");
        _ = tokens.next();       // label
        var seeds = std.ArrayList(usize).init(allocator);

        while (tokens.next()) |seed| {
            if (seed.len > 0) {
                try seeds.append(try std.fmt.parseUnsigned(usize, seed, 10));
            }
        }

        break :blk seeds;
    };

    var mappings = std.StringHashMap(Map).init(allocator);

    // Skip empty line
    try reader.skipUntilDelimiterOrEof('\n');

    while (true) {
        var label = try allocator.dupe(u8, reader.readUntilDelimiter(&buf, ' ') catch break);
        try reader.skipUntilDelimiterOrEof('\n');

        // Read one or more maps
        var ranges = std.ArrayList(MapRange).init(allocator);

        while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
            if (line.len == 0) {
                break;
            }

            var it = std.mem.splitAny(u8, line, " ");

            var dst_start = try std.fmt.parseUnsigned(usize, it.next().?, 10);
            var src_start = try std.fmt.parseUnsigned(usize, it.next().?, 10);
            var len = try std.fmt.parseUnsigned(usize, it.next().?, 10);

            try ranges.append(MapRange {
                .dst_start = dst_start,
                .src_start = src_start,
                .src_end = src_start + len,
                .len = len,
            });
        }

        try mappings.put(label, try Map.new(allocator, &ranges.items));
    }

    var lowest_location: usize = std.math.maxInt(usize);

    var map0 = mappings.getPtr("seed-to-soil").?;
    var map1 = mappings.getPtr("soil-to-fertilizer").?;
    var map2 = mappings.getPtr("fertilizer-to-water").?;
    var map3 = mappings.getPtr("water-to-light").?;
    var map4 = mappings.getPtr("light-to-temperature").?;
    var map5 = mappings.getPtr("temperature-to-humidity").?;
    var map6 = mappings.getPtr("humidity-to-location").?;

    var i: usize = 0;
    while (i < seed_ranges.items.len): (i += 2) {
        var range_start = seed_ranges.items[i];
        var range_len = seed_ranges.items[i + 1];

        var r: usize = 0;
        while (r < range_len): (r += 1) {
            var seed = range_start + r;

            var mapped: usize = seed;

            mapped = map0.map(mapped);
            mapped = map1.map(mapped);
            mapped = map2.map(mapped);
            mapped = map3.map(mapped);
            mapped = map4.map(mapped);
            mapped = map5.map(mapped);
            mapped = map6.map(mapped);

            if (mapped < lowest_location) {
                std.debug.print("New best: {d}\n", . {
                    mapped
                });
                lowest_location = mapped;
            }
        }
    }

    std.debug.print("Part 2 lowest location was {d}\n", . {
        lowest_location
    });
}


pub fn day4Pt1() !void {
    var buf: [1024]u8 = undefined;

    var file = try std.fs.cwd().openFile("input_files/day4.txt", .{ .mode = std.fs.File.OpenMode.read_only });

    var reader = file.reader();

    var total_score: usize = 0;

    while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var score: usize = 0;

        var numbers = blk: {
            var it = std.mem.splitSequence(u8, line, ": ");
            _ = it.next();
            break :blk it.next().?;
        };

        var sides = std.mem.splitSequence(u8, numbers, " | ");
        var winning_numbers: std.StaticBitSet(256) = std.StaticBitSet(256).initEmpty();
        {
            var it = std.mem.window(u8, sides.next().?, 2, 3);

            while (it.next()) |s| {
                var winner = try std.fmt.parseUnsigned(usize, std.mem.trim(u8, s, " "), 10);
                winning_numbers.set(winner);
            }
        }

        var it = std.mem.window(u8, sides.next().?, 2, 3);
        while (it.next()) |s| {
            var our_number = try std.fmt.parseUnsigned(usize, std.mem.trim(u8, s, " "), 10);

            if (winning_numbers.isSet(our_number)) {
                if (score == 0) {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }

        total_score += score;
    }

    std.debug.print("Part 1 total score was: {d}\n", .{total_score});

}

pub fn day4Pt2() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    var allocator = arena.allocator();

    var buf: [1024]u8 = undefined;

    var file = try std.fs.cwd().openFile("input_files/day4.txt", .{ .mode = std.fs.File.OpenMode.read_only });

    var reader = file.reader();

    var card_scores = std.ArrayList(usize).init(allocator);

    while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var score: usize = 0;

        var numbers = blk: {
            var it = std.mem.splitSequence(u8, line, ": ");
            _ = it.next();
            break :blk it.next().?;
        };

        var sides = std.mem.splitSequence(u8, numbers, " | ");
        var winning_numbers: std.StaticBitSet(256) = std.StaticBitSet(256).initEmpty();
        {
            var it = std.mem.window(u8, sides.next().?, 2, 3);

            while (it.next()) |s| {
                var winner = try std.fmt.parseUnsigned(usize, std.mem.trim(u8, s, " "), 10);
                winning_numbers.set(winner);
            }
        }

        var it = std.mem.window(u8, sides.next().?, 2, 3);
        while (it.next()) |s| {
            var our_number = try std.fmt.parseUnsigned(usize, std.mem.trim(u8, s, " "), 10);

            if (winning_numbers.isSet(our_number)) {
                score += 1;
            }
        }

        try card_scores.append(score);
    }

    var queue = std.ArrayList(usize).init(allocator);
    var cards_handled: usize = 0;

    {
        var i: usize = 0;
        while (i < card_scores.items.len): (i += 1) {
            try queue.append(i);
        }
    }

    while (queue.items.len > 0) {
        var next_item = queue.pop();
        cards_handled += 1;

        var card_score = card_scores.items[next_item];

        var i: usize = 0;
        while (i < card_score): (i += 1) {
            var next_idx = next_item + 1 + i;

            if (next_idx < card_scores.items.len) {
                try queue.append(next_idx);
            }
        }
    }

    std.debug.print("Part 2 we won a total of {d} cards\n", .{ cards_handled } );
}


const Coord2d = struct {
    row: usize,
    col: usize,
};

pub fn day3Pt1() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    var allocator = arena.allocator();

    var file = try std.fs.cwd().openFile("input_files/day3.txt", .{ .mode = std.fs.File.OpenMode.read_only });
    var bytes = try file.readToEndAlloc(allocator, std.math.maxInt(usize));

    var width: usize = 0;
    var height: usize = 0;
    var grid = std.ArrayList([]const u8).init(allocator);

    var line_it = std.mem.splitSequence(u8, std.mem.trim(u8, bytes, "\n"), "\n");
    while (line_it.next()) |line| {
        width = line.len;
        height += 1;
        try grid.append(line);
    }

    var grid_dims = Coord2d { .row = height, .col = width };

    std.debug.print("{d} x {d}\n", .{width, height});

    var positions_of_interest = std.AutoHashMap(Coord2d, void).init(allocator);

    {
        var row: usize = 0;
        while (row < grid_dims.row): (row += 1) {
            var col: usize = 0;
            while (col < grid_dims.col): (col += 1) {
                var ch = grid.items[row][col];
                if (!std.ascii.isDigit(ch) and ch != '.') {
                    // Punctuation of interest
                    for ([_]i8 { -1, 0, 1}) |row_off| {
                        for ([_]i8 { -1, 0, 1}) |col_off| {
                            try positions_of_interest.put(Coord2d {
                                .row = @intCast(@as(isize, @intCast(row)) + row_off),
                                .col = @intCast(@as(isize, @intCast(col)) + col_off)
                                }, {});
                        }
                    }
                }
            }
        }
    }

    // Walk the grid, find the numbers we care about, add 'em up
    var total: usize = 0;
    {
        var row: usize = 0;
        while (row < grid_dims.row): (row += 1) {
            var col: usize = 0;
            while (col < grid_dims.col): (col += 1) {
                if (std.ascii.isDigit(grid.items[row][col])) {
                    var is_counted = false;
                    var value: usize = 0;

                    while (col < grid_dims.col and std.ascii.isDigit(grid.items[row][col])): (col += 1) {
                        if (positions_of_interest.contains(Coord2d { .row = row, .col = col })) {
                            is_counted = true;
                        }

                        value *= 10;
                        value += grid.items[row][col] - '0';
                    }

                    if (is_counted) {
                        total += value;
                    }
                }
            }
        }
    }

    std.debug.print("Part 1 total was: {d}\n", . {
        total
    });
}

pub fn day3Pt2() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    var allocator = arena.allocator();

    var file = try std.fs.cwd().openFile("input_files/day3.txt", .{ .mode = std.fs.File.OpenMode.read_only });
    var bytes = try file.readToEndAlloc(allocator, std.math.maxInt(usize));

    var width: usize = 0;
    var height: usize = 0;
    var grid = std.ArrayList([]const u8).init(allocator);

    var line_it = std.mem.splitSequence(u8, std.mem.trim(u8, bytes, "\n"), "\n");
    while (line_it.next()) |line| {
        width = line.len;
        height += 1;
        try grid.append(line);
    }

    var grid_dims = Coord2d { .row = height, .col = width };

    std.debug.print("{d} x {d}\n", .{width, height});

    var positions_of_interest = std.AutoHashMap(Coord2d, std.ArrayList(usize)).init(allocator);

    var gear_ratios = std.ArrayList(usize).init(allocator);
    var gear_factor_count = std.ArrayList(usize).init(allocator);

    {
        var row: usize = 0;
        while (row < grid_dims.row): (row += 1) {
            var col: usize = 0;
            while (col < grid_dims.col): (col += 1) {
                var ch = grid.items[row][col];
                if (ch == '*') {
                    try gear_ratios.append(1);
                    try gear_factor_count.append(0);
                    var gear_id = gear_ratios.items.len - 1;

                    // Record our gear ID against the positions of interest
                    //
                    // Sometimes one number correspond to two gears, so watching for that too...
                    for ([_]i8 { -1, 0, 1}) |row_off| {
                        for ([_]i8 { -1, 0, 1}) |col_off| {
                            var coord = Coord2d {
                                .row = @intCast(@as(isize, @intCast(row)) + row_off),
                                .col = @intCast(@as(isize, @intCast(col)) + col_off)
                            };

                            if (!positions_of_interest.contains(coord)) {
                                try positions_of_interest.put(coord, std.ArrayList(usize).init(allocator));
                            }

                            var coord_positions = positions_of_interest.getPtr(coord).?;
                            try coord_positions.*.append(gear_id);
                        }
                    }
                }
            }
        }
    }

    // Walk the grid, find the numbers we care about, add them to our gear ratios
    {
        var row: usize = 0;
        while (row < grid_dims.row): (row += 1) {
            var col: usize = 0;
            while (col < grid_dims.col): (col += 1) {
                if (std.ascii.isDigit(grid.items[row][col])) {
                    var gear_ids = try std.DynamicBitSet.initEmpty(allocator, 0);
                    var value: usize = 0;

                    while (col < grid_dims.col and std.ascii.isDigit(grid.items[row][col])): (col += 1) {
                        var coord = Coord2d { .row = row, .col = col };

                        if (positions_of_interest.contains(coord)) {
                            for (positions_of_interest.get(coord).?.items) |gear_id| {
                                if (gear_ids.capacity() < (gear_id + 1)) {
                                    try gear_ids.resize(gear_id + 1, false);
                                }
                                gear_ids.set(gear_id);
                            }
                        }

                        value *= 10;
                        value += grid.items[row][col] - '0';
                    }

                    var bits = gear_ids.iterator(.{});
                    while (bits.next()) |gear_id| {
                        gear_ratios.items[gear_id] *= value;
                        gear_factor_count.items[gear_id] += 1;
                    }
                }
            }
        }
    }

    var total: usize = 0;
    var idx: usize = 0;
    while (idx < gear_factor_count.items.len): (idx += 1) {
        if (gear_factor_count.items[idx] == 2) {
            total += gear_ratios.items[idx];
        }
    }

    std.debug.print("Part 2 total gear ratio: {}\n", .{total});
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
