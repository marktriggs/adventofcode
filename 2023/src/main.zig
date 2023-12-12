// (cd /mnt/ssd/mst-home/projects/adventofcode/2023/src && zig build-exe -O ReleaseFast main.zig && ./main)

const std = @import("std");

pub fn main() !void {
    // try Day1.day1Pt1();
    // try Day1.day1Pt2();
    //
    // try Day2.day2Pt1();
    // try Day2.day2Pt2();
    //
    // try Day3.day3Pt1();
    // try Day3.day3Pt2();
    //
    // try Day4.day4Pt1();
    // try Day4.day4Pt2();
    //
    // try Day5.day5Pt1();
    // try Day5.day5Pt2();
    //
    // try Day6.day6Pt1();
    // try Day6.day6Pt2();
    //
    // try Day7.Pt1.day7Pt1();
    // try Day7.Pt2.day7Pt2();

    // try Day8.Pt1.day8Pt1();
    // try Day8.Pt2.day8Pt2();

    // try Day9.Pt1.day9Pt1();
    // try Day9.Pt2.day9Pt2();

    // try Day10.day10Pt1();
    // try Day10.day10Pt2();

    // try Day11.day11Pt1();
    // try Day11.day11Pt2();

    try Day12.Pt1.day12Pt1();
    try Day12.Pt2.day12Pt2();
}

const Day12 = struct {
    const Pt1 = struct {
        pub fn day12Pt1() !void {
            var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
            var allocator = arena.allocator();

            var total: usize = 0;

            var file = try std.fs.cwd().openFile("input_files/day12.txt", .{ .mode = std.fs.File.OpenMode.read_only });
            var reader = file.reader();
            var buf: [1024]u8 = undefined;
            while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
                var it = std.mem.splitSequence(u8, line, " ");
                var lhs = it.next().?;
                var rhs = it.next().?;

                var groups = std.ArrayList(usize).init(allocator);

                var ns = std.mem.splitSequence(u8, rhs, ",");
                while (ns.next()) |s| {
                    try groups.append(try std.fmt.parseUnsigned(usize, s, 10));
                }

                total += countArrangements(try allocator.dupe(u8, lhs), groups.items);
            }

            std.debug.print("Part 1: arrangements {d}\n", .{total});
        }


        fn countArrangements(remaining_input: []u8, remaining_groups: []usize) usize {
            if (remaining_groups.len == 0) {
                var matches = true;
                for (remaining_input) |ch| {
                    if (ch == '#') {
                        matches = false;
                    }
                }

                if (matches) {
                    return 1;
                } else {
                    return 0;
                }
            }

            if (remaining_input.len == 0) {
                return 0;
            }

            if (remaining_groups.len == 0) {
                return 0;
            }

            var next_group = remaining_groups[0];

            if (remaining_input.len < next_group) {
                return 0;
            }

            if (remaining_input[0] == '#') {
                var matches = true;
                {
                    var i: usize = 0;
                    while (i < next_group): (i += 1) {
                        if (remaining_input[i] == '.') {
                            matches = false;
                        }
                    }
                }

                if (matches and (remaining_input.len == next_group or remaining_input[next_group] != '#')) {
                    if (remaining_input.len == next_group) {
                        if (remaining_groups.len == 1) {
                            return 1;
                        } else {
                            return 0;
                        }
                    } else {
                        return countArrangements(remaining_input[(next_group + 1)..], remaining_groups[1..]);
                    }
                } else {
                    return 0;
                }
            } else if (remaining_input[0] == '.') {
                return countArrangements(remaining_input[1..], remaining_groups);
            } else if (remaining_input[0] == '?') {
                var matches = true;
                {
                    var i: usize = 0;
                    while (i < next_group): (i += 1) {
                        if (remaining_input[i] == '.') {
                            matches = false;
                        }
                    }
                }

                if (matches and (remaining_input.len == next_group or remaining_input[next_group] != '#')) {
                    if (remaining_input.len == next_group) {
                        if (remaining_groups.len == 1) {
                            return 1;
                        } else {
                            return 0;
                        }
                    } else {
                        return countArrangements(remaining_input[(next_group + 1)..], remaining_groups[1..]) + countArrangements(remaining_input[1..], remaining_groups);
                    }
                } else {
                    return countArrangements(remaining_input[1..], remaining_groups);
                }
            } else {
                unreachable;
            }
        }

    };

    const Pt2 = struct {
        pub fn day12Pt2() !void {
            var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
            var allocator = arena.allocator();

            var total: usize = 0;

            var cache = std.ArrayList(?usize).init(allocator);
            {
                var i: usize = 0;
                while (i <= 65535): (i += 1) {
                    try cache.append(null);
                }
            }


            var file = try std.fs.cwd().openFile("input_files/day12.txt", .{ .mode = std.fs.File.OpenMode.read_only });
            var reader = file.reader();
            var buf: [1024]u8 = undefined;
            while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
                var it = std.mem.splitSequence(u8, line, " ");
                var lhs = it.next().?;
                var rhs = it.next().?;

                var groups = std.ArrayList(usize).init(allocator);

                {
                    var i: usize = 0;
                    while (i < 5): (i += 1) {
                        var ns = std.mem.splitSequence(u8, rhs, ",");
                        while (ns.next()) |s| {
                            try groups.append(try std.fmt.parseUnsigned(usize, s, 10));
                        }
                    }
                }

                // Part 2: unfold
                var expandedLHS = std.ArrayList(u8).init(allocator);

                {
                    var i: usize = 0;
                    while (i < 5): (i += 1) {
                        try expandedLHS.appendSlice(lhs);
                        if (i != 4) {
                            try expandedLHS.appendSlice("?");
                        }
                    }
                }

                {
                    var i: usize = 0;
                    while (i <= 65535): (i += 1) {
                        cache.items[i] = null;
                    }
                }

                total += countArrangementsMemo(cache.items, expandedLHS.items, groups.items);
            }

            std.debug.print("Part 2: arrangements {d}\n", .{total});
        }

        fn countArrangementsMemo(cache: []?usize, remaining_input: []u8, remaining_groups: []usize) usize {
            std.debug.assert(remaining_input.len < 256);
            std.debug.assert(remaining_groups.len < 256);

            var cache_key = remaining_input.len << 8 | remaining_groups.len;

            if (cache[cache_key] != null) {
                return cache[cache_key].?;
            }

            var result = countArrangements(cache, remaining_input, remaining_groups);

            cache[cache_key] = result;

            return result;
        }

        fn countArrangements(cache: []?usize, remaining_input: []u8, remaining_groups: []usize) usize {
            if (remaining_groups.len == 0) {
                var matches = true;
                for (remaining_input) |ch| {
                    if (ch == '#') {
                        matches = false;
                    }
                }

                if (matches) {
                    return 1;
                } else {
                    return 0;
                }
            }

            if (remaining_input.len == 0) {
                return 0;
            }

            if (remaining_groups.len == 0) {
                return 0;
            }

            var next_group = remaining_groups[0];

            if (remaining_input.len < next_group) {
                return 0;
            }

            if (remaining_input[0] == '#') {
                var matches = true;
                {
                    var i: usize = 0;
                    while (i < next_group): (i += 1) {
                        if (remaining_input[i] == '.') {
                            matches = false;
                        }
                    }
                }

                if (matches and (remaining_input.len == next_group or remaining_input[next_group] != '#')) {
                    if (remaining_input.len == next_group) {
                        if (remaining_groups.len == 1) {
                            return 1;
                        } else {
                            return 0;
                        }
                    } else {
                        return countArrangementsMemo(cache, remaining_input[(next_group + 1)..], remaining_groups[1..]);
                    }
                } else {
                    return 0;
                }
            } else if (remaining_input[0] == '.') {
                return countArrangementsMemo(cache, remaining_input[1..], remaining_groups);
            } else if (remaining_input[0] == '?') {
                var matches = true;
                {
                    var i: usize = 0;
                    while (i < next_group): (i += 1) {
                        if (remaining_input[i] == '.') {
                            matches = false;
                        }
                    }
                }

                if (matches and (remaining_input.len == next_group or remaining_input[next_group] != '#')) {
                    if (remaining_input.len == next_group) {
                        if (remaining_groups.len == 1) {
                            return 1;
                        } else {
                            return 0;
                        }
                    } else {
                        return countArrangementsMemo(cache, remaining_input[(next_group + 1)..], remaining_groups[1..]) + countArrangementsMemo(cache, remaining_input[1..], remaining_groups);
                    }
                } else {
                    return countArrangementsMemo(cache, remaining_input[1..], remaining_groups);
                }
            } else {
                unreachable;
            }
        }
    };

};

const Day11 = struct {
    const Galaxy = struct {
        id: usize,
        row: usize,
        col: usize,
    };

    pub fn day11Pt1() !void {
        var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
        var allocator = arena.allocator();

        var file = try std.fs.cwd().openFile("input_files/day11.txt", .{ .mode = std.fs.File.OpenMode.read_only });
        var reader = file.reader();
        var buf: [1024]u8 = undefined;

        var populated_rows = try std.DynamicBitSet.initEmpty(allocator, 0);
        var populated_cols = try std.DynamicBitSet.initEmpty(allocator, 0);

        var galaxies = std.ArrayList(Galaxy).init(allocator);

        var next_id: usize = 1;

        var row: usize = 0;
        while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line|: (row += 1) {
            try populated_rows.resize(row + 1, false);
            try populated_cols.resize(line.len + 1, false);

            var col: usize = 0;
            while (col < line.len): (col += 1) {


                if (line[col] == '#') {
                    populated_rows.set(row);
                    populated_cols.set(col);

                    try galaxies.append(Galaxy {
                        .id = next_id,
                        .row = row,
                        .col = col,
                    });

                    next_id += 1;
                }
            }
        }

        // Expand our galaxy
        var galaxy_idx: usize = 0;
        while (galaxy_idx < galaxies.items.len): (galaxy_idx += 1) {
            var galaxy = &galaxies.items[galaxy_idx];

            var row_adjustment: usize = 0;
            {
                var i: usize = 0;
                while (i < galaxy.row): (i += 1) {
                    if (!populated_rows.isSet(i)) {
                        row_adjustment += 1;
                    }
                }
            }

            var col_adjustment: usize = 0;
            {
                var i: usize = 0;
                while (i < galaxy.col): (i += 1) {
                    if (!populated_cols.isSet(i)) {
                        col_adjustment += 1;
                    }
                }
            }

            galaxy.row += row_adjustment;
            galaxy.col += col_adjustment;
        }

        // Calculate shortest paths
        var total: usize = 0;

        var a: usize = 0;
        while (a < galaxies.items.len): (a += 1) {
            var b: usize = a + 1;

            while (b < galaxies.items.len): (b += 1) {
                // std.debug.print("{d} to {d}: {d}\n",
                //                 .{
                //                     galaxies.items[a].id,
                //                     galaxies.items[b].id,
                //                     @abs(@as(isize, @intCast(galaxies.items[a].row)) - @as(isize, @intCast(galaxies.items[b].row))) +
                //                         @abs(@as(isize, @intCast(galaxies.items[a].col)) - @as(isize, @intCast(galaxies.items[b].col)))
                // });

                total += (@abs(@as(isize, @intCast(galaxies.items[a].row)) - @as(isize, @intCast(galaxies.items[b].row))) +
                              @abs(@as(isize, @intCast(galaxies.items[a].col)) - @as(isize, @intCast(galaxies.items[b].col))));
            }
        }

        std.debug.print("Part 1 Total distances: {d}\n", .{total});
    }

    pub fn day11Pt2() !void {
        var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
        var allocator = arena.allocator();

        var file = try std.fs.cwd().openFile("input_files/day11.txt", .{ .mode = std.fs.File.OpenMode.read_only });
        var reader = file.reader();
        var buf: [1024]u8 = undefined;

        var populated_rows = try std.DynamicBitSet.initEmpty(allocator, 0);
        var populated_cols = try std.DynamicBitSet.initEmpty(allocator, 0);

        var galaxies = std.ArrayList(Galaxy).init(allocator);

        var next_id: usize = 1;

        var row: usize = 0;
        while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line|: (row += 1) {
            try populated_rows.resize(row + 1, false);
            try populated_cols.resize(line.len + 1, false);

            var col: usize = 0;
            while (col < line.len): (col += 1) {


                if (line[col] == '#') {
                    populated_rows.set(row);
                    populated_cols.set(col);

                    try galaxies.append(Galaxy {
                        .id = next_id,
                        .row = row,
                        .col = col,
                    });

                    next_id += 1;
                }
            }
        }

        // Expand our galaxy
        var galaxy_idx: usize = 0;
        while (galaxy_idx < galaxies.items.len): (galaxy_idx += 1) {
            var galaxy = &galaxies.items[galaxy_idx];

            var row_adjustment: usize = 0;
            {
                var i: usize = 0;
                while (i < galaxy.row): (i += 1) {
                    if (!populated_rows.isSet(i)) {
                        row_adjustment += 999999;
                    }
                }
            }

            var col_adjustment: usize = 0;
            {
                var i: usize = 0;
                while (i < galaxy.col): (i += 1) {
                    if (!populated_cols.isSet(i)) {
                        col_adjustment += 999999;
                    }
                }
            }

            galaxy.row += row_adjustment;
            galaxy.col += col_adjustment;
        }

        // Calculate shortest paths
        var total: usize = 0;

        var a: usize = 0;
        while (a < galaxies.items.len): (a += 1) {
            var b: usize = a + 1;

            while (b < galaxies.items.len): (b += 1) {
                // std.debug.print("{d} to {d}: {d}\n",
                //                 .{
                //                     galaxies.items[a].id,
                //                     galaxies.items[b].id,
                //                     @abs(@as(isize, @intCast(galaxies.items[a].row)) - @as(isize, @intCast(galaxies.items[b].row))) +
                //                         @abs(@as(isize, @intCast(galaxies.items[a].col)) - @as(isize, @intCast(galaxies.items[b].col)))
                // });

                total += (@abs(@as(isize, @intCast(galaxies.items[a].row)) - @as(isize, @intCast(galaxies.items[b].row))) +
                              @abs(@as(isize, @intCast(galaxies.items[a].col)) - @as(isize, @intCast(galaxies.items[b].col))));
            }
        }

        std.debug.print("Part 2 Total distances: {d}\n", .{total});
    }
};


const Day10 = struct {
    const Tile = enum(u8) {
        Vertical,
        Horizontal,
        NEBend,
        NWBend,
        SWBend,
        SEBend,
        Ground,
        Start,
    };

    const Grid = struct {
        width: usize,
        height: usize,

        start_row: usize,
        start_col: usize,

        rows: [][]Tile,

        fn deriveStartType(self: *const Grid) Tile {
            std.debug.assert(self.rows[self.start_row][self.start_col] == Tile.Start);

            var top =    if (self.start_row == 0 )                Tile.Ground  else self.rows[self.start_row - 1][self.start_col];
            var bottom = if ((self.start_row + 1) == self.height) Tile.Ground  else self.rows[self.start_row + 1][self.start_col];
            var left =   if (self.start_col == 0)                 Tile.Ground  else self.rows[self.start_row][self.start_col - 1];
            var right =  if ((self.start_col + 1) == self.width)  Tile.Ground  else self.rows[self.start_row][self.start_col + 1];

            if (top == Tile.Vertical     and right == Tile.Horizontal) { return Tile.NEBend; }
            if (top == Tile.Vertical     and right == Tile.NWBend)     { return Tile.NEBend; }
            if (top == Tile.Vertical     and right == Tile.SWBend)     { return Tile.NEBend; }
            if (top == Tile.SWBend       and right == Tile.Horizontal) { return Tile.NEBend; }
            if (top == Tile.SWBend       and right == Tile.NWBend)     { return Tile.NEBend; }
            if (top == Tile.SWBend       and right == Tile.SWBend)     { return Tile.NEBend; }
            if (top == Tile.SEBend       and right == Tile.Horizontal) { return Tile.NEBend; }
            if (top == Tile.SEBend       and right == Tile.NWBend)     { return Tile.NEBend; }
            if (top == Tile.SEBend       and right == Tile.SWBend)     { return Tile.NEBend; }
            if (top == Tile.Vertical     and bottom == Tile.Vertical)  { return Tile.Vertical; }
            if (top == Tile.Vertical     and bottom == Tile.NEBend)    { return Tile.Vertical; }
            if (top == Tile.Vertical     and bottom == Tile.NWBend)    { return Tile.Vertical; }
            if (top == Tile.SWBend       and bottom == Tile.Vertical)  { return Tile.Vertical; }
            if (top == Tile.SWBend       and bottom == Tile.NEBend)    { return Tile.Vertical; }
            if (top == Tile.SWBend       and bottom == Tile.NWBend)    { return Tile.Vertical; }
            if (top == Tile.SEBend       and bottom == Tile.Vertical)  { return Tile.Vertical; }
            if (top == Tile.SEBend       and bottom == Tile.NEBend)    { return Tile.Vertical; }
            if (top == Tile.SEBend       and bottom == Tile.NWBend)    { return Tile.Vertical; }
            if (top == Tile.Vertical     and left == Tile.Horizontal)  { return Tile.NWBend; }
            if (top == Tile.Vertical     and left == Tile.NEBend)      { return Tile.NWBend; }
            if (top == Tile.Vertical     and left == Tile.SEBend)      { return Tile.NWBend; }
            if (top == Tile.SWBend       and left == Tile.Horizontal)  { return Tile.NWBend; }
            if (top == Tile.SWBend       and left == Tile.NEBend)      { return Tile.NWBend; }
            if (top == Tile.SWBend       and left == Tile.SEBend)      { return Tile.NWBend; }
            if (top == Tile.SEBend       and left == Tile.Horizontal)  { return Tile.NWBend; }
            if (top == Tile.SEBend       and left == Tile.NEBend)      { return Tile.NWBend; }
            if (top == Tile.SEBend       and left == Tile.SEBend)      { return Tile.NWBend; }
            if (right == Tile.Horizontal and bottom == Tile.Vertical)  { return Tile.SEBend; }
            if (right == Tile.Horizontal and bottom == Tile.NEBend)    { return Tile.SEBend; }
            if (right == Tile.Horizontal and bottom == Tile.NWBend)    { return Tile.SEBend; }
            if (right == Tile.NWBend     and bottom == Tile.Vertical)  { return Tile.SEBend; }
            if (right == Tile.NWBend     and bottom == Tile.NEBend)    { return Tile.SEBend; }
            if (right == Tile.NWBend     and bottom == Tile.NWBend)    { return Tile.SEBend; }
            if (right == Tile.SWBend     and bottom == Tile.Vertical)  { return Tile.SEBend; }
            if (right == Tile.SWBend     and bottom == Tile.NEBend)    { return Tile.SEBend; }
            if (right == Tile.SWBend     and bottom == Tile.NWBend)    { return Tile.SEBend; }
            if (right == Tile.Horizontal and left == Tile.Horizontal)  { return Tile.Horizontal; }
            if (right == Tile.Horizontal and left == Tile.NEBend)      { return Tile.Horizontal; }
            if (right == Tile.Horizontal and left == Tile.SEBend)      { return Tile.Horizontal; }
            if (right == Tile.NWBend     and left == Tile.Horizontal)  { return Tile.Horizontal; }
            if (right == Tile.NWBend     and left == Tile.NEBend)      { return Tile.Horizontal; }
            if (right == Tile.NWBend     and left == Tile.SEBend)      { return Tile.Horizontal; }
            if (right == Tile.SWBend     and left == Tile.Horizontal)  { return Tile.Horizontal; }
            if (right == Tile.SWBend     and left == Tile.NEBend)      { return Tile.Horizontal; }
            if (right == Tile.SWBend     and left == Tile.SEBend)      { return Tile.Horizontal; }
            if (bottom == Tile.Vertical  and left == Tile.Horizontal)  { return Tile.SWBend; }
            if (bottom == Tile.Vertical  and left == Tile.NEBend)      { return Tile.SWBend; }
            if (bottom == Tile.Vertical  and left == Tile.SEBend)      { return Tile.SWBend; }
            if (bottom == Tile.NEBend    and left == Tile.Horizontal)  { return Tile.SWBend; }
            if (bottom == Tile.NEBend    and left == Tile.NEBend)      { return Tile.SWBend; }
            if (bottom == Tile.NEBend    and left == Tile.SEBend)      { return Tile.SWBend; }
            if (bottom == Tile.NWBend    and left == Tile.Horizontal)  { return Tile.SWBend; }
            if (bottom == Tile.NWBend    and left == Tile.NEBend)      { return Tile.SWBend; }
            if (bottom == Tile.NWBend    and left == Tile.SEBend)      { return Tile.SWBend; }

            unreachable;
        }

        fn nextPossibleLocations(self: *const Grid, location: Point) ![2]Point {
            var current_tile_type = self.rows[location.row][location.col];

            if (current_tile_type == Tile.Start) {
                current_tile_type = self.deriveStartType();
            }

            switch (current_tile_type) {
                Tile.Vertical => {
                    return [_]Point { location.adjust(-1, 0), location.adjust(1, 0) };
                },
                Tile.Horizontal => {
                    return [_]Point { location.adjust(0, -1), location.adjust(0, 1) };
                },
                Tile.NEBend => {
                    return [_]Point { location.adjust(-1, 0), location.adjust(0, 1) };
                },
                Tile.NWBend => {
                    return [_]Point { location.adjust(-1, 0), location.adjust(0, -1) };
                },
                Tile.SWBend => {
                    return [_]Point { location.adjust(1, 0), location.adjust(0, -1) };
                },
                Tile.SEBend => {
                    return [_]Point { location.adjust(1, 0), location.adjust(0, 1) };
                },
                else => {
                    std.debug.print("Unexpected current tile: {any} at location row={d} col={d}\n", .{current_tile_type, location.row, location.col});
                    unreachable;
                },
            }
        }
    };

    fn readGrid(allocator: std.mem.Allocator, path: []const u8) !Grid {
        var file = try std.fs.cwd().openFile(path, .{ .mode = std.fs.File.OpenMode.read_only });
        var reader = file.reader();
        var buf: [1024]u8 = undefined;

        var rows = std.ArrayList([]Tile).init(allocator);

        var start_row: usize = 0;
        var start_col: usize = 0;

        while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
            var row = std.ArrayList(Tile).init(allocator);

            var i: usize = 0;
            while (i < line.len): (i += 1) {
                var tile = switch (line[i]) {
                    '|' => Tile.Vertical,
                    '-' => Tile.Horizontal,
                    'L' => Tile.NEBend,
                    'J' => Tile.NWBend,
                    '7' => Tile.SWBend,
                    'F' => Tile.SEBend,
                    '.' => Tile.Ground,
                    'S' => Tile.Start,
                    else => unreachable,
                };

                if (tile == Tile.Start) {
                    start_row = rows.items.len;
                    start_col = row.items.len;
                }

                try row.append(tile);
            }

            try rows.append(row.items);
        }

        return Grid {
            .width = rows.items[0].len,
            .height = rows.items.len,
            .rows = rows.items,
            .start_row = start_row,
            .start_col = start_col,
        };
    }

    const Point = struct {
        row: usize,
        col: usize,

        fn adjust(self: *const Point, row_adjust: isize, col_adjust: isize) Point {
            var new_row = @as(isize, @intCast(self.row)) + row_adjust;
            var new_col = @as(isize, @intCast(self.col)) + col_adjust;

            return .{
                .row = @intCast(new_row),
                .col = @intCast(new_col),
            };
        }
    };

    pub fn day10Pt1() !void {
        var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
        var allocator = arena.allocator();

        var grid = try readGrid(allocator, "input_files/day10.txt");

        // Measure the loop length
        var visited_locations = std.AutoHashMap(Point, bool).init(allocator);

        var location = Point { .row = grid.start_row, .col = grid.start_col };

        var steps: usize = 0;
        while (true) {
            try visited_locations.put(location, true);
            var next_locations = try grid.nextPossibleLocations(location);

            var moved: bool = false;
            for (next_locations) |loc| {
                if (!visited_locations.contains(loc)) {
                    location = loc;
                    moved = true;
                    break;
                }
            }

            steps += 1;

            if (!moved) {
                break;
            }
        }

        std.debug.print("Completed loop in {d} steps\n", .{steps});
        std.debug.print("Furthest distance from origin is {d} steps\n", .{steps / 2});
    }

    fn sprite(row1: []const u8, row2: []const u8, row3: []const u8, row4: []const u8) [16]u8 {
        var result: [16]u8 = undefined;

        std.mem.copy(u8, result[0..], row1);
        std.mem.copy(u8, result[4..], row2);
        std.mem.copy(u8, result[8..], row3);
        std.mem.copy(u8, result[12..], row4);

        return result;
    }

    fn loadShapes() [][16]u8 {
        var result: [255][16]u8 = undefined;

        result[@intFromEnum(Tile.Vertical)] = sprite(".##.",
                                                     ".##.",
                                                     ".##.",
                                                     ".##.");

        result[@intFromEnum(Tile.Horizontal)] = sprite("....",
                                                       "####",
                                                       "####",
                                                       "....");

        result[@intFromEnum(Tile.NEBend)] = sprite(".##.",
                                                   ".###",
                                                   ".###",
                                                   "....");

        result[@intFromEnum(Tile.NWBend)] = sprite(".##.",
                                                   "###.",
                                                   "###.",
                                                   "....");

        result[@intFromEnum(Tile.SWBend)] = sprite("....",
                                                   "###.",
                                                   "###.",
                                                   ".##.");

        result[@intFromEnum(Tile.SEBend)] = sprite("....",
                                                   ".###",
                                                   ".###",
                                                   ".##.");

        return &result;
    }

    pub fn day10Pt2() !void {
        var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
        var allocator = arena.allocator();

        var grid = try readGrid(allocator, "input_files/day10.txt");

        // Measure the loop length
        var visited_locations = std.AutoHashMap(Point, bool).init(allocator);

        var location = Point { .row = grid.start_row, .col = grid.start_col };

        var steps: usize = 0;
        while (true) {
            try visited_locations.put(location, true);
            var next_locations = try grid.nextPossibleLocations(location);

            var moved: bool = false;
            for (next_locations) |loc| {
                if (!visited_locations.contains(loc)) {
                    location = loc;
                    moved = true;
                    break;
                }
            }

            steps += 1;

            if (!moved) {
                break;
            }
        }

        var shapes = loadShapes();

        var blank_tile = sprite("xxxx",
                                "xxxx",
                                "xxxx",
                                "xxxx");

        // width x height x pixels per tile x RGBA
        var bitmap = std.ArrayList(u8).init(allocator);
        var px_size = grid.width * 4 * grid.height * 4 * 4;
        while (px_size > 0): (px_size -= 1) {
            try bitmap.append(0);
        }

        var r_idx: usize = 0;
        while (r_idx < grid.height): (r_idx += 1) {
            var c_idx: usize = 0;
            while (c_idx < grid.width): (c_idx += 1) {
                var shape = blank_tile;

                if (visited_locations.contains(Point { .row = r_idx, .col = c_idx})) {
                    var tile = grid.rows[r_idx][c_idx];

                    if (tile == Tile.Start) {
                        tile = grid.deriveStartType();
                    }

                    shape = shapes[@intFromEnum(tile)];
                }


                // Draw our shape
                var char_row_idx: usize = 0;
                while (char_row_idx < 4): (char_row_idx += 1) {
                    var char_col_idx: usize = 0;
                    while (char_col_idx < 4): (char_col_idx += 1) {
                        var ch = shape[(char_row_idx * 4) + char_col_idx];

                        var colour: u32 = 0xFFFFFFFF;

                        if (ch == 'x') {
                            colour = 0x666666FF;
                        } else if (ch == '#') {
                            colour = 0xFF0000FF;
                        }

                        var write_pos = (r_idx * 4 * grid.width * 16) + (char_row_idx * grid.width * 16) + (c_idx * 4 * 4) + (char_col_idx * 4);

                        bitmap.items[write_pos] = @intCast(colour >> 24 & 0xff);
                        bitmap.items[write_pos + 1] = @intCast(colour >> 16 & 0xff);
                        bitmap.items[write_pos + 2] = @intCast(colour >> 8 & 0xff);
                        bitmap.items[write_pos + 3] = @intCast(colour >> 0 & 0xff);
                    }
                }
            }
        }

        std.debug.print("Writing {d}x{d} bitmap\n", .{ grid.width * 4, grid.height * 4 });
        var out = try std.fs.createFileAbsolute("/home/mst/tmp/grid.data", .{ });
        defer out.close();

        _ = try out.write(bitmap.items);

        var file = try std.fs.openFileAbsolute("/home/mst/tmp/grid-bucketed.data", .{ .mode = std.fs.File.OpenMode.read_only });
        var buf: [4]u8 = undefined;

        var contained_pixels: usize = 0;

        while (true) {
            var len = try file.read(&buf);

            if (len != 4) {
                break;
            }

            if (std.mem.eql(u8, &buf, &[_]u8 { 0x66, 0x66, 0x66, 0xFF })) {
                contained_pixels += 1;
            }
        }

        std.debug.print("I count {d} contained pixels\n", .{contained_pixels / 16});
    }
};


const Day9 = struct {
    const Pt1 = struct {

        fn nextInSequence(allocator: std.mem.Allocator, sequence: []isize) !isize {
            var last_sequence = sequence;
            var differences = std.ArrayList([]isize).init(allocator);

            try differences.append(sequence);

            // Calculate the differences until we hit all zeroes
            while (true) {
                var successive_differences = std.ArrayList(isize).init(allocator);

                var all_zeroes: bool = true;

                var i: usize = 1;
                while (i < last_sequence.len): (i += 1) {
                    var difference = last_sequence[i] - last_sequence[i - 1];
                    try successive_differences.append(difference);

                    all_zeroes = all_zeroes and (difference == 0);
                }

                try differences.append(successive_differences.items);

                if (all_zeroes) {
                    break;
                }

                last_sequence = successive_differences.items;
            }

            // Extrapolate upwards
            var next_elt: isize = 0;

            for (differences.items) |ds| {
                next_elt += ds[ds.len - 1];
            }

            return next_elt;
        }


        pub fn day9Pt1() !void {
            var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
            var allocator = arena.allocator();

            var file = try std.fs.cwd().openFile("input_files/day9.txt", .{ .mode = std.fs.File.OpenMode.read_only });
            var reader = file.reader();
            var buf: [1024]u8 = undefined;

            var total: isize = 0;

            while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
                var tokens = std.mem.tokenizeAny(u8, line, " ");
                var sequence = std.ArrayList(isize).init(allocator);

                while (tokens.next()) |n| {
                        try sequence.append(try std.fmt.parseInt(isize, n, 10));
                }

                var next_in_sequence = try nextInSequence(allocator, sequence.items);

                total += next_in_sequence;
            }

            std.debug.print("Total: {d}\n", . {
                total
            });
        }
    };

    const Pt2 = struct {

        fn nextInSequence(allocator: std.mem.Allocator, sequence: []isize) !isize {
            var last_sequence = sequence;
            var differences = std.ArrayList([]isize).init(allocator);

            try differences.append(sequence);

            // Calculate the differences until we hit all zeroes
            while (true) {
                var successive_differences = std.ArrayList(isize).init(allocator);

                var all_zeroes: bool = true;

                var i: usize = 1;
                while (i < last_sequence.len): (i += 1) {
                    var difference = last_sequence[i] - last_sequence[i - 1];
                    try successive_differences.append(difference);

                    all_zeroes = all_zeroes and (difference == 0);
                }

                try differences.append(successive_differences.items);

                if (all_zeroes) {
                    break;
                }

                last_sequence = successive_differences.items;
            }

            // Extrapolate upwards and BACKWARDS
            var next_elt: isize = 0;

            var idx = differences.items.len - 2;

            while (true) {
                // std.debug.print("  {d} - {d} - {d}\n", .{differences.items[idx][0], next_elt, differences.items[idx][0] - next_elt});
                next_elt = differences.items[idx][0] - next_elt;

                if (idx > 0) {
                    idx -= 1;
                } else {
                    break;
                }
            }

            return next_elt;
        }


        pub fn day9Pt2() !void {
            var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
            var allocator = arena.allocator();

            var file = try std.fs.cwd().openFile("input_files/day9.txt", .{ .mode = std.fs.File.OpenMode.read_only });
            var reader = file.reader();
            var buf: [1024]u8 = undefined;

            var total: isize = 0;

            while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
                var tokens = std.mem.tokenizeAny(u8, line, " ");
                var sequence = std.ArrayList(isize).init(allocator);

                while (tokens.next()) |n| {
                        try sequence.append(try std.fmt.parseInt(isize, n, 10));
                }

                var next_in_sequence = try nextInSequence(allocator, sequence.items);

                // std.debug.print("{s} ... {d}\n", .{line, next_in_sequence});

                total += next_in_sequence;
            }

            std.debug.print("Total: {d}\n", . {
                total
            });
        }
    };
};


const Day8 = struct {

    // Dumping a bunch of notes in here because I ended up solving PT2 by hand:


    // Path 4 first cycles at node295 idx4 after 12087 steps with cycle length of 12083.  End is 12079 steps from start of cycle (cycle contains 1 end nodes)
    // Path 5 first cycles at node349 idx2 after 13209 steps with cycle length of 13207.  End is 13205 steps from start of cycle (cycle contains 1 end nodes)
    // Path 2 first cycles at node35 idx2 after 17143 steps with cycle length of 17141.  End is 17139 steps from start of cycle (cycle contains 1 end nodes)
    // Path 1 first cycles at node699 idx4 after 18831 steps with cycle length of 18827.  End is 18823 steps from start of cycle (cycle contains 1 end nodes)
    // Path 0 first cycles at node122 idx7 after 20520 steps with cycle length of 20513.  End is 20506 steps from start of cycle (cycle contains 1 end nodes)
    // Path 3 first cycles at node324 idx2 after 22201 steps with cycle length of 22199.  End is 22197 steps from start of cycle (cycle contains 1 end nodes)
    //
    // Path 4 reached end at T 24166
    // Path 5 reached end at T 26414
    // Path 2 reached end at T 34282
    // Path 1 reached end at T 37654
    // Path 0 reached end at T 41026
    // Path 3 reached end at T 44398
    //
    //
    // let's look at T = 24166
    //
    //   Path 4 is at its end (cycle start: 12087; position in cycle (- 24166 12087) 12079 which is end)
    //   Path 5 is (cycle start: 13209; position in cycle (- 24166 13209) 10957 which is (- 13205 10957) 2248 steps away from end)
    //   Path 2 is (cycle start: 17143; position in cycle (- 24166 17143) 7023 which is (- 17139 7023) 10116 steps away from end)
    //   Path 1 is (cycle start: 18831; position in cycle (- 24166 18831) 5335 which is (- 18823 5335) 13488 steps away from end)
    //   Path 0 is (cycle start: 20520; position in cycle (- 24166 20520) 3646 which is (- 20506 3646) 16860 steps away from end)
    //   Path 3 is (cycle start: 22201; position in cycle (- 24166 22201) 1965 which is (- 22197 1965) 20232 steps away from end)
    //
    // path 4 has a cycle length of 12083
    //
    //
    //
    // Paths we're tracking:
    //
    // Path 4 first cycles at node295 idx4 after 12087 steps with cycle length of 12083.  End is 12079 steps from start of cycle (cycle contains 1 end nodes)
    // Path 5 first cycles at node349 idx2 after 13209 steps with cycle length of 13207.  End is 13205 steps from start of cycle (cycle contains 1 end nodes)
    // Path 2 first cycles at node35 idx2 after 17143 steps with cycle length of 17141.  End is 17139 steps from start of cycle (cycle contains 1 end nodes)
    // Path 1 first cycles at node699 idx4 after 18831 steps with cycle length of 18827.  End is 18823 steps from start of cycle (cycle contains 1 end nodes)
    // Path 0 first cycles at node122 idx7 after 20520 steps with cycle length of 20513.  End is 20506 steps from start of cycle (cycle contains 1 end nodes)
    // Path 3 first cycles at node324 idx2 after 22201 steps with cycle length of 22199.  End is 22197 steps from start of cycle (cycle contains 1 end nodes)
    //
    // Path 4: cycle start: 12087.  cycle length 12083 steps.  End is 12079 steps from start of cycle
    // Path 5: cycle start: 13209.  cycle length 13207 steps.  End is 13205 steps from start of cycle
    // Path 2: cycle start: 17143.  cycle length 17141 steps.  End is 17139 steps from start of cycle
    // Path 1: cycle start: 18831.  cycle length 18827 steps.  End is 18823 steps from start of cycle
    // Path 0: cycle start: 20520.  cycle length 20513 steps.  End is 20506 steps from start of cycle
    // Path 3: cycle start: 22201.  cycle length 22199 steps.  End is 22197 steps from start of cycle
    //
    //
    // at T = 24166, Path 4 hits its end position for the first time.  At this time:
    //
    // Path 4 is at position (- 24166 12087) 12079 in its cycle
    // Path 5 is at position (- 24166 13209) 10957 in its cycle
    // Path 2 is at position (- 24166 17143) 7023 in its cycle
    // Path 1 is at position (- 24166 18831) 5335 in its cycle
    // Path 0 is at position (- 24166 20520) 3646 in its cycle
    // Path 3 is at position (- 24166 22201) 1965 in its cycle
    //
    //
    // Path 5's end position is 13205 steps from the start of its cycle.  We can keep adding multiples of Path 4's cycle length (12083) to advance Path 5
    //
    // We are done when:
    //
    //   10957 + (12083 * N) % 13207 == 13205
    //
    // That's N = 45
    //
    // So we advance T += (* 12083 45) 543735
    //
    // At T = (+ 24166 543735) = 567901
    //
    // Path 4 is at position 12079
    // Path 5 is at position 13205
    //
    // Path 2 has moved as well.  It was at 7023, and now it's at:
    //
    //   (7023 + 543735) % 17141  = 2246
    //
    // Path 2 will be in position when:
    //
    //   2246 + (LCM(12083, 13207) * N) % 17141 = 17139
    //
    // LCM(12083, 13207) is 567901
    //
    // Solving for N...
    //
    // N = 1124
    //
    // So we advance T += (* 1124 567901) = 638320724
    //
    // At T = (+ 567901 638320724) 638888625
    //
    // Path 4 is at position 12079
    // Path 5 is at position 13205
    // Path 2 is at position 17139
    //
    // Path 1 has moved.  Back at T = 24166, it was at position 5335, and now it's at
    //
    //   (% (+ 5535 543735 638320724) 18827) 13403
    //
    // Path 1 will be in position when:
    //
    //   13403 + (LCM(12083, 13207, 17141) * N) % 13403 = 18823
    //
    // LCM(12083, 13207, 17141) = 34641961
    //
    // Solving for N
    //
    //
    // NOTE: At this point, solving for N in the above turned out to be hard.  Seems
    // like it was a very large N.  But then I noticed that:
    //
    // LCM(12083, 13207) is 567901
    //
    // and that was exactly the amount I'd worked out to tick T by.  Could it be
    // that the T value we're looking for is just the LCM of all of the cycle
    // lengths?
    //
    // Answer: YES INDEED.  It seems that the input was engineered to make this work
    // out, but besides noticing the pattern and having a guess, I'm still not sure
    // what the "right" way of knowing this was.
    //
    //
    // Part2: 13,385,272,668,829
    //
    const Pt1 = struct {
        const Direction = struct {
            left: []u8,
            right: []u8,
        };

        pub fn day8Pt1() !void {
            var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
            var allocator = arena.allocator();

            var file = try std.fs.cwd().openFile("input_files/day8.txt", .{ .mode = std.fs.File.OpenMode.read_only });

            var reader = file.reader();
            var buf: [1024]u8 = undefined;

            var directions = try allocator.dupe(u8, (try reader.readUntilDelimiterOrEof(&buf, '\n')).?);

            // Skip the blank line
            _ = try reader.readUntilDelimiterOrEof(&buf, '\n');

            var mappings = std.StringHashMap(Direction).init(allocator);

            while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
                var tokens = std.mem.tokenizeAny(u8, line, " =(),");

                var src = try allocator.dupe(u8, tokens.next().?);
                var lhs = try allocator.dupe(u8, tokens.next().?);
                var rhs = try allocator.dupe(u8, tokens.next().?);

                try mappings.put(src, Direction { .left = lhs, .right = rhs });
            }

            var step_count: usize = 0;

            var idx: usize = 0;
            var current_node: []const u8 = "AAA";
            while (!std.mem.eql(u8, current_node, "ZZZ")) {
                step_count += 1;
                var next_mapping = mappings.getPtr(current_node).?;
                if (directions[idx] == 'L') {
                    current_node = next_mapping.left;
                } else {
                    current_node = next_mapping.right;
                }

                idx += 1;
                if (idx >= directions.len) {
                    idx = 0;
                }
            }

            std.debug.print("Part 1: found the exit in {d} steps\n", .{step_count});
        }
    };

    const Pt2 = struct {
        const Direction = struct {
            left: u16,
            right: u16,
        };

        const Mappings = struct {
            input: []u8,
            start_set: std.StaticBitSet(65536),
            end_set: std.StaticBitSet(65536),
            map: []?Direction,
        };

        fn readMappings(allocator: std.mem.Allocator, path: []const u8) !Mappings {
            // Pass 1: intern all strings
            var string_table = std.StringHashMap(u16).init(allocator);
            var start_set = std.StaticBitSet(65536).initEmpty();
            var end_set = std.StaticBitSet(65536).initEmpty();

            {
                var file = try std.fs.cwd().openFile(path, .{ .mode = std.fs.File.OpenMode.read_only });
                var reader = file.reader();
                var buf: [1024]u8 = undefined;

                // Skip the directions and blank line
                _ = try reader.readUntilDelimiterOrEof(&buf, '\n');
                _ = try reader.readUntilDelimiterOrEof(&buf, '\n');

                var next_string_id: u16 = 0;

                while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
                    var tokens = std.mem.tokenizeAny(u8, line, " =(),");

                    var src = try allocator.dupe(u8, tokens.next().?);
                    var lhs = try allocator.dupe(u8, tokens.next().?);
                    var rhs = try allocator.dupe(u8, tokens.next().?);

                    if (!string_table.contains(src)) {
                        try string_table.put(src, next_string_id);
                        next_string_id += 1;
                    }

                    if (std.mem.endsWith(u8, src, "A")) {
                        start_set.set(string_table.get(src).?);
                    }

                    if (std.mem.endsWith(u8, src, "Z")) {
                        end_set.set(string_table.get(src).?);
                    }


                    if (!string_table.contains(lhs)) {
                        try string_table.put(lhs, next_string_id);
                        next_string_id += 1;
                    }

                    if (!string_table.contains(rhs)) {
                        try string_table.put(rhs, next_string_id);
                        next_string_id += 1;
                    }
                }
            }

            // Pass 2: load our mappings
            var file = try std.fs.cwd().openFile(path, .{ .mode = std.fs.File.OpenMode.read_only });
            var reader = file.reader();
            var buf: [1024]u8 = undefined;

            var directions = try allocator.dupe(u8, (try reader.readUntilDelimiterOrEof(&buf, '\n')).?);

            // Skip the blank line
            _ = try reader.readUntilDelimiterOrEof(&buf, '\n');

            var mappings = std.ArrayList(?Direction).init(allocator);

            var keys = string_table.keyIterator();
            while (keys.next()) |_| {
                try mappings.append(null);
            }

            while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
                var tokens = std.mem.tokenizeAny(u8, line, " =(),");

                var src = string_table.get(tokens.next().?).?;
                var lhs = string_table.get(tokens.next().?).?;
                var rhs = string_table.get(tokens.next().?).?;

                mappings.items[src] = Direction { .left = lhs, .right = rhs };
            }

            return Mappings {
                .input = directions,
                .start_set = start_set,
                .end_set = end_set,
                .map = mappings.items,
            };
        }

        const Path = struct {
            current_node: u16,
            visited: std.DynamicBitSet,
            cycle_length: ?usize,
            start_cycle_key: ?u32,
            start_cycle_steps: ?usize,
            steps_to_end: ?usize,
            end_node_count_in_cycle: usize,
        };

        pub fn day8Pt2() !void {
            var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
            var allocator = arena.allocator();

            var mappings = try readMappings(allocator, "input_files/day8.txt");

            var idx: usize = 0;
            var step_count: usize = 0;

            var paths = std.ArrayList(Path).init(allocator);

            var bits = mappings.start_set.iterator(.{});
            while (bits.next()) |start_id| {
                try paths.append(Path {
                    .current_node = @intCast(start_id),
                    .visited = try std.DynamicBitSet.initEmpty(allocator, std.math.maxInt(u32)),
                    .cycle_length = null,
                    .start_cycle_key = null,
                    .start_cycle_steps = null,
                    .steps_to_end = null,
                    .end_node_count_in_cycle = 0,
                });
            }

            while (true) {
                var path_idx: usize = 0;
                var all_cycled = true;
                while (path_idx < paths.items.len): (path_idx += 1) {
                    var path = &paths.items[path_idx];

                    if (path.cycle_length != null) {
                        continue;
                    }


                    var position_key: u32 = (@as(u32, path.current_node) << 16) | @as(u32, @intCast(idx));
                    // std.debug.print("{d} + {d} = {d}\n", .{path.current_node, idx, position_key});
                    if (path.start_cycle_key == null) {
                        if (path.visited.isSet(position_key)) {
                            path.start_cycle_key = @intCast(position_key);
                            path.start_cycle_steps = step_count;
                        } else {
                            path.visited.set(position_key);
                        }
                    } else {
                        if (mappings.end_set.isSet(path.current_node)) {
                            path.end_node_count_in_cycle += 1;
                            path.steps_to_end = step_count - path.start_cycle_steps.?;
                            std.debug.print("Path {d} reached end at T {d}\n", .{path_idx, step_count});
                        }

                        if (position_key == path.start_cycle_key.?) {
                            path.cycle_length = step_count - path.start_cycle_steps.?;
                            std.debug.print("Path {d} first cycles at node{d} idx{d} after {d} steps with cycle length of {d}.  End is {d} steps from start of cycle (cycle contains {d} end nodes)\n", .{
                                path_idx,
                                path.current_node,
                                idx,
                                path.start_cycle_steps.?,
                                path.cycle_length.?,
                                path.steps_to_end.?,
                                path.end_node_count_in_cycle,
                            });
                        }
                    }

                    if (path.cycle_length == null) {
                        all_cycled = false;
                    }

                    var next_mapping = &mappings.map[path.current_node].?;
                    if (mappings.input[idx] == 'L') {
                        path.current_node = next_mapping.left;
                    } else if (mappings.input[idx] == 'R')  {
                        path.current_node = next_mapping.right;
                    } else {
                        unreachable;
                    }
                }

                if (all_cycled) {
                    break;
                }

                step_count += 1;
                idx += 1;
                if (idx >= mappings.input.len) {
                    idx = 0;
                }
            }
        }
    };
};


const Day7 = struct {

    const HandType = enum(u8) {
        high_card,
        one_pair,
        two_pair,
        three_of_a_kind,
        full_house,
        four_of_a_kind,
        five_of_a_kind,
    };


    const Pt1 = struct {
        const Hand = struct {
            const CardOrdering = "23456789TJQKA";

            cards: []const u8,

            fn handType(self: *const Hand) HandType {
                var card_frequencies = std.mem.zeroes([256]usize);

                for (self.cards) |card| {
                    card_frequencies[card] += 1;
                }

                var frequency_frequencies = std.mem.zeroes([6]usize);

                for (card_frequencies) |freq| {
                    if (freq > 0) {
                        frequency_frequencies[freq] += 1;
                    }
                }

                if (frequency_frequencies[5] == 1) { return HandType.five_of_a_kind; }
                if (frequency_frequencies[4] == 1) { return HandType.four_of_a_kind; }
                if (frequency_frequencies[3] == 1 and frequency_frequencies[2] == 1) { return HandType.full_house; }
                if (frequency_frequencies[3] == 1) { return HandType.three_of_a_kind; }
                if (frequency_frequencies[2] == 2) { return HandType.two_pair; }
                if (frequency_frequencies[2] == 1) { return HandType.one_pair; }

                return HandType.high_card;
            }

            fn isLessThan(self: *const Hand, other: *const Hand) bool {
                var self_type = self.handType();
                var other_type = other.handType();

                if (self_type == other_type) {
                    var i: usize = 0;
                    while (i < self.cards.len): (i += 1) {
                        if (std.mem.indexOfScalar(u8, CardOrdering, self.cards[i]).? == std.mem.indexOfScalar(u8, CardOrdering, other.cards[i]).?) {
                            // continue
                        } else {
                            return std.mem.indexOfScalar(u8, CardOrdering, self.cards[i]).? < std.mem.indexOfScalar(u8, CardOrdering, other.cards[i]).?;
                        }
                    }
                } else {
                    return @intFromEnum(self_type) < @intFromEnum(other_type);
                }

                unreachable();
            }
        };

        const HandBid = struct {
            hand: Hand,
            bid: usize,

            fn compareHandBid(context: void, a: HandBid, b: HandBid) bool {
                _ = context;

                return a.hand.isLessThan(&b.hand);
            }
        };


        pub fn day7Pt1() !void {
            var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
            var allocator = arena.allocator();

            var file = try std.fs.cwd().openFile("input_files/day7.txt", .{ .mode = std.fs.File.OpenMode.read_only });

            var buf: [1024]u8 = undefined;
            var reader = file.reader();
            var hand_bids = std.ArrayList(HandBid).init(allocator);

            while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
                var it = std.mem.splitSequence(u8, line, " ");

                var cards = it.next().?;
                var bid = it.next().?;

                try hand_bids.append(HandBid {
                    .hand = Hand { .cards = try allocator.dupe(u8, cards) },
                    .bid = try std.fmt.parseUnsigned(usize, bid, 10),
                });
            }

            std.sort.heap(HandBid, hand_bids.items, {}, HandBid.compareHandBid);

            var total: usize  = 0;
            var count: usize  = 0;
            for (hand_bids.items) |hand_bid| {
                count += 1;

                std.debug.print("{s} - {any} - bid:{d} - ({d} * {d})\n", .{
                    hand_bid.hand.cards,
                    hand_bid.hand.handType(),
                    hand_bid.bid,
                    count,
                    hand_bid.bid,
                });

                total += hand_bid.bid * count;
            }

            std.debug.print("Part 1 total: {}\n", . {
                total
            });
        }
    };


    const Pt2 = struct {
        const Hand = struct {
            const CardOrdering = "J23456789TQKA";

            cards: []const u8,

            fn handType(self: *const Hand) HandType {
                var card_frequencies = std.mem.zeroes([256]usize);

                var wildcard_count: usize = 0;

                for (self.cards) |card| {
                    if (card == 'J') {
                        wildcard_count += 1;
                    } else {
                        card_frequencies[card] += 1;
                    }
                }

                // Apply wildcards: just boost the highest card frequency until we run out
                while (wildcard_count > 0): (wildcard_count -= 1) {
                    var max_frequency: usize = 0;

                    for (card_frequencies) |f| {
                        if (f > max_frequency) {
                            max_frequency = f;
                        }
                    }

                    var i: usize = 0;
                    while (i < card_frequencies.len): (i += 1) {
                        if (card_frequencies[i] == max_frequency) {
                            card_frequencies[i] += 1;
                            break;
                        }
                    }
                }

                var frequency_frequencies = std.mem.zeroes([6]usize);

                for (card_frequencies) |freq| {
                    if (freq > 0) {
                        frequency_frequencies[freq] += 1;
                    }
                }

                if (frequency_frequencies[5] == 1) { return HandType.five_of_a_kind; }
                if (frequency_frequencies[4] == 1) { return HandType.four_of_a_kind; }
                if (frequency_frequencies[3] == 1 and frequency_frequencies[2] == 1) { return HandType.full_house; }
                if (frequency_frequencies[3] == 1) { return HandType.three_of_a_kind; }
                if (frequency_frequencies[2] == 2) { return HandType.two_pair; }
                if (frequency_frequencies[2] == 1) { return HandType.one_pair; }

                return HandType.high_card;
            }

            fn isLessThan(self: *const Hand, other: *const Hand) bool {
                var self_type = self.handType();
                var other_type = other.handType();

                if (self_type == other_type) {
                    var i: usize = 0;
                    while (i < self.cards.len): (i += 1) {
                        if (std.mem.indexOfScalar(u8, CardOrdering, self.cards[i]).? == std.mem.indexOfScalar(u8, CardOrdering, other.cards[i]).?) {
                            // continue
                        } else {
                            return std.mem.indexOfScalar(u8, CardOrdering, self.cards[i]).? < std.mem.indexOfScalar(u8, CardOrdering, other.cards[i]).?;
                        }
                    }
                } else {
                    return @intFromEnum(self_type) < @intFromEnum(other_type);
                }

                unreachable();
            }
        };

        const HandBid = struct {
            hand: Hand,
            bid: usize,

            fn compareHandBid(context: void, a: HandBid, b: HandBid) bool {
                _ = context;

                return a.hand.isLessThan(&b.hand);
            }
        };


        pub fn day7Pt2() !void {
            var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
            var allocator = arena.allocator();

            var file = try std.fs.cwd().openFile("input_files/day7.txt", .{ .mode = std.fs.File.OpenMode.read_only });

            var buf: [1024]u8 = undefined;
            var reader = file.reader();
            var hand_bids = std.ArrayList(HandBid).init(allocator);

            while (try reader.readUntilDelimiterOrEof(&buf, '\n')) |line| {
                var it = std.mem.splitSequence(u8, line, " ");

                var cards = it.next().?;
                var bid = it.next().?;

                try hand_bids.append(HandBid {
                    .hand = Hand { .cards = try allocator.dupe(u8, cards) },
                    .bid = try std.fmt.parseUnsigned(usize, bid, 10),
                });
            }

            std.sort.heap(HandBid, hand_bids.items, {}, HandBid.compareHandBid);

            var total: usize  = 0;
            var count: usize  = 0;
            for (hand_bids.items) |hand_bid| {
                count += 1;

                std.debug.print("{s} - {any} - bid:{d} - ({d} * {d})\n", .{
                    hand_bid.hand.cards,
                    hand_bid.hand.handType(),
                    hand_bid.bid,
                    count,
                    hand_bid.bid,
                });

                total += hand_bid.bid * count;
            }

            std.debug.print("Part 2 total: {}\n", . {
                total
            });
        }
    };

};


const Day6 = struct {
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

};


const Day5 = struct {
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
};


const Day4 = struct {
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
};


const Day3 = struct {
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
};

const Day2 = struct {
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
};

const Day1 = struct {
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
};

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
