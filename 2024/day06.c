#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <assert.h>

#include "lib.c"

typedef enum {
    North = 1,
    East = 2,
    South = 4,
    West = 8,
} Direction;

typedef struct {
    char bytes[32768];
    int width;
    int height;
} Grid;

typedef struct {
    int row;
    int col;
    Direction direction;
} GuardLocation;

typedef struct {
    Grid grid;

    GuardLocation guard;
} Input;


Input load_input() {
    FILE *fh = fopen("input_files/day6.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    fseek(fh, 0L, SEEK_END);
    size_t size = (size_t)ftell(fh);
    rewind(fh);

    Input result = { 0 };

    assert(size < sizeof result.grid.bytes);

    size_t read_bytes = fread(result.grid.bytes, 1, size, fh);

    assert(read_bytes == size);

    fclose(fh);

    while (result.grid.bytes[result.grid.width] != '\n') {
        result.grid.width++;
    }

    for (int i = 0; i < (int)size; i++) {
        if (result.grid.bytes[i] == '\n') {
            result.grid.height++;
        }
    }

    result.guard.row = -1;
    result.guard.col = -1;
    result.guard.direction = North;


    for (int y = 0; y < result.grid.height; y++) {
        for (int x = 0; x < result.grid.width; x++) {
            char ch = result.grid.bytes[y * (result.grid.width + 1) + x];

            if (ch == '^') {
                result.guard.row = y;
                result.guard.col = x;

                result.grid.bytes[y * (result.grid.width + 1) + x] = '.';
            }
        }
    }

    assert(result.guard.col >= 0 && result.guard.row >= 0);

    return result;
}


void grid_clear(Grid *g) {
    bzero(g->bytes, sizeof(g->bytes));
}

void grid_set(Grid *g, int row, int col, char value) {
    g->bytes[row * (g->width + 1) + col] = value;
}

void grid_or(Grid *g, int row, int col, char value) {
    g->bytes[row * (g->width + 1) + col] |= value;
}

char grid_get(Grid *g, int row, int col) {
    int idx = row * (g->width + 1) + col;

    if(idx >= 0 && idx < (int)sizeof(g->bytes)) {
        return g->bytes[idx];
    } else {
        return '\0';
    }
}

GuardLocation move_guard(GuardLocation guard) {
    int next_guard_row = guard.row;
    int next_guard_col = guard.col;

    switch (guard.direction) {
    case North:
        next_guard_row -= 1;
        break;
    case East:
        next_guard_col += 1;
        break;
    case South:
        next_guard_row += 1;
        break;
    case West:
        next_guard_col -= 1;
        break;
    }

    return (GuardLocation) {
        .row = next_guard_row,
        .col = next_guard_col,
        .direction = guard.direction,
    };
}

GuardLocation turn_right(GuardLocation guard) {
    Direction new_direction = guard.direction;

    switch (guard.direction) {
    case North:
        new_direction = East;
        break;
    case East:
        new_direction = South;
        break;
    case South:
        new_direction = West;
        break;
    case West:
        new_direction = North;
        break;
    }

    return (GuardLocation) {
        .row = guard.row,
        .col = guard.col,
        .direction = new_direction,
    };
}

typedef struct {
    Grid walked_positions;
    int looped;
} SimulationResult;

SimulationResult simulate_guard(Grid map, GuardLocation guard) {
    Grid walked_positions = map;
    grid_clear(&walked_positions);

    int looped = 0;

    for (;;) {
        if (guard.row >= map.height || guard.row < 0 ||
            guard.col >= map.width || guard.col < 0) {
            // We're done
            break;
        }

       if (grid_get(&walked_positions, guard.row, guard.col) & (int)guard.direction) {
            // We've looped
            looped = 1;
            break;
        }

        grid_or(&walked_positions, guard.row, guard.col, (char)guard.direction);

        GuardLocation next_guard_location = move_guard(guard);

        if (grid_get(&map, next_guard_location.row, next_guard_location.col) == '#') {
            // Blocked!
            next_guard_location = turn_right(guard);
        }

        guard = next_guard_location;
    }

    return (SimulationResult) {
        .walked_positions = walked_positions,
        .looped = looped,
    };
}

void part1() {
    Input input = load_input();

    GuardLocation guard = input.guard;

    SimulationResult result = simulate_guard(input.grid, guard);

    int squares_reached = 0;

    for (int y = 0; y < result.walked_positions.height; y++) {
        for (int x = 0; x < result.walked_positions.width; x++) {
            if (grid_get(&result.walked_positions, y, x)) {
                squares_reached++;
            }
        }
    }

    printf("Part 1: Guard reached %d squares\n", squares_reached);
}

void part2() {
    Input input = load_input();

    // Simulate the initial map to find the guard's path
    GuardLocation guard = input.guard;
    SimulationResult result = simulate_guard(input.grid, guard);
    assert(!result.looped);

    int loops_created = 0;

    // Try blocking each square on the path and see if we loop
    for (int y = 0; y < result.walked_positions.height; y++) {
        for (int x = 0; x < result.walked_positions.width; x++) {
            if (x == guard.col && y == guard.row) {
                // We don't obstruct the guard's starting position
                continue;
            }

            if (grid_get(&result.walked_positions, y, x)) {
                Grid modified_grid = input.grid;
                grid_set(&modified_grid, y, x, '#');

                SimulationResult obstructed_result = simulate_guard(modified_grid, guard);

                if (obstructed_result.looped) {
                    loops_created++;
                }
            }
        }
    }

    printf("Part 2: Found %d positions to create a loop\n", loops_created);
}

int main() {
    part1();
    part2();

    return 0;
}
