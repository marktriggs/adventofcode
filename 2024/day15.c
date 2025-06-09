#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <assert.h>
#include <stdint.h>
#include <sys/stat.h>
#include <sys/types.h>

#include "lib.c"

#define MAX_GRID 8192
#define MAX_INSTRUCTIONS 32768

typedef struct {
    int x;
    int y;
} Point;

typedef struct {
    char grid[MAX_GRID];
    int width;
    int height;

    Point robot;

    char instructions[MAX_INSTRUCTIONS];
    int instruction_count;
} Input;

Input load_input(int use_part2_adjustment) {
    FILE *fh = fopen("input_files/day15.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    Input result = { 0 };

    char buf[4096];

    // Read grid
    if (use_part2_adjustment) {
        int row = 0;
        int offset = 0;
        while (fgets(buf, sizeof(buf) - 1, fh)) {
            assert(offset < MAX_GRID);
            if (buf[0] == '\n') {
                break;
            }

            int i;
            for (i = 0; buf[i] != '\n'; i++) {
                if (buf[i] == 'O') {
                    result.grid[offset++] = '[';
                    result.grid[offset++] = ']';
                } else if (buf[i] == '@') {
                    result.robot.x = i * 2;
                    result.robot.y = row;

                    result.grid[offset++] = '@';
                    result.grid[offset++] = '.';
                } else {
                    result.grid[offset++] = buf[i];
                    result.grid[offset++] = buf[i];
                }
            }

            result.width = i * 2;
            result.height++;
            row++;
        }
    } else {
        int row = 0;
        int offset = 0;

        while (fgets(buf, sizeof(buf) - 1, fh)) {
            if (buf[0] == '\n') {
                break;
            }

            int i;
            for (i = 0; buf[i] != '\n'; i++) {
                if (buf[i] == '@') {
                    result.robot.x = i;
                    result.robot.y = row;
                }

                result.grid[offset++] = buf[i];
            }

            result.width = i;
            result.height++;
            row++;
        }
    }

    // Read instructions
    while (fgets(buf, sizeof(buf) - 1, fh)) {
        if (buf[0] == '\n') {
            break;
        }

        for (int i = 0; buf[i] != '\n'; i++) {
            result.instructions[result.instruction_count++] = buf[i];
        }
    }

    fclose(fh);

    return result;
}

Point point_add(Point p1, Point p2) {
    return (Point) {
        .x = p1.x + p2.x,
        .y = p1.y + p2.y,
    };
}


int in_range(Input *input, Point p) {
    return (p.x >= 0 && p.y >= 0 && p.x < input->width && p.y < input->height);
}

Point move(Point to_move, Point offset, Input *input) {
    Point adjusted = point_add(to_move, offset);

    if (!in_range(input, adjusted)) {
        // Didn't move
        return to_move;
    }

    char target_cell_occupant = input->grid[(adjusted.y * input->width) + adjusted.x];

    if (target_cell_occupant == '#') {
        // Hit a wall
        return to_move;
    }

    if (target_cell_occupant == '.') {
        // Move in
        input->grid[(adjusted.y * input->width) + adjusted.x] = input->grid[(to_move.y * input->width) + to_move.x];
        input->grid[(to_move.y * input->width) + to_move.x] = '.';

        return adjusted;
    }

    // Try making space and retry
    move(adjusted, offset, input);

    target_cell_occupant = input->grid[(adjusted.y * input->width) + adjusted.x];

    if (target_cell_occupant == '.') {
        // Move in
        input->grid[(adjusted.y * input->width) + adjusted.x] = input->grid[(to_move.y * input->width) + to_move.x];
        input->grid[(to_move.y * input->width) + to_move.x] = '.';

        return adjusted;
    } else {
        // Couldn't move
        return to_move;
    }
}

void show_grid(Input *input) {
    for (int row = 0; row < input->height; row++) {
        for (int col = 0; col < input->width; col++) {
            printf("%c", input->grid[(row * input->width) + col]);
        }
        printf("\n");
    }
}

void part1() {
    Input input = load_input(0);

    for (int move_idx = 0; move_idx < input.instruction_count; move_idx++) {
        Point offset = { 0 };

        switch (input.instructions[move_idx]) {
        case '^':
            offset.y -= 1;
            break;
        case '>':
            offset.x += 1;
            break;
        case '<':
            offset.x -= 1;
            break;
        case 'v':
            offset.y += 1;
            break;
        }

        input.robot = move(input.robot, offset, &input);
    }

    int gps = 0;

    for (int row = 0; row < input.height; row++) {
        for (int col = 0; col < input.width; col++){
            if (input.grid[(row * input.width) + col] == 'O') {
                gps += (100 * row) + col;
            }
        }
    }

    printf("Part 1 GPS value: %d\n", gps);
}

Point move_pt2(Point to_move, Point offset, Input *input) {
    Point adjusted = point_add(to_move, offset);

    if (!in_range(input, adjusted)) {
        // Didn't move
        return to_move;
    }

    int is_vertical_move = offset.y != 0;
    char target_cell_occupant = input->grid[(adjusted.y * input->width) + adjusted.x];

    if (target_cell_occupant == '#') {
        // Hit a wall
        return to_move;
    }

    char moving_guy = input->grid[(to_move.y * input->width) + to_move.x];

    if (moving_guy == '#') {
        return to_move;
    }

    if (is_vertical_move) {
        assert(moving_guy != '.');

        if (moving_guy == '@') {
            // Moving the robot, which only takes one space
            if (target_cell_occupant != '.') {
                move_pt2(adjusted, offset, input);
                target_cell_occupant = input->grid[(adjusted.y * input->width) + adjusted.x];
            }

            if (target_cell_occupant == '.') {
                // Move in
                input->grid[(adjusted.y * input->width) + adjusted.x] = input->grid[(to_move.y * input->width) + to_move.x];
                input->grid[(to_move.y * input->width) + to_move.x] = '.';

                return adjusted;
            } else {
                // Couldn't move
                return to_move;
            }
        } else {
            // Moving a block, which has two parts now
            Point other_point = to_move;
            if (moving_guy == '[') {
                other_point.x += 1;
            } else {
                other_point.x -= 1;
            }

            assert(input->grid[(other_point.y * input->width) + other_point.x] == ']' ||  input->grid[(other_point.y * input->width) + other_point.x] == '[');

            Input test_input = *input;
            if (target_cell_occupant != '.') {
                move_pt2(adjusted, offset, &test_input);
            }

            Point adjusted_other_point = point_add(other_point, offset);
            char other_target_cell_occupant = test_input.grid[(adjusted_other_point.y * input->width) + adjusted_other_point.x];

            if (other_target_cell_occupant != '.') {
                move_pt2(adjusted_other_point, offset, &test_input);
            }

            if (test_input.grid[(adjusted.y * input->width) + adjusted.x] != '.' ||
                test_input.grid[(adjusted_other_point.y * input->width) + adjusted_other_point.x] != '.') {
                // We couldn't clear enough space
                return to_move;
            }

            // Do the actual moves
            if (target_cell_occupant != '.') {
                move_pt2(adjusted, offset, input);
            }

            other_target_cell_occupant = input->grid[(adjusted_other_point.y * input->width) + adjusted_other_point.x];
            if (other_target_cell_occupant != '.') {
                move_pt2(adjusted_other_point, offset, input);
            }

            // Move in
            input->grid[(adjusted.y * input->width) + adjusted.x] = input->grid[(to_move.y * input->width) + to_move.x];
            input->grid[(to_move.y * input->width) + to_move.x] = '.';
            input->grid[(adjusted_other_point.y * input->width) + adjusted_other_point.x] = input->grid[(other_point.y * input->width) + other_point.x];
            input->grid[(other_point.y * input->width) + other_point.x] = '.';

            // Whew!
            return adjusted;
        }
    } else {
        // easier case...
        if (target_cell_occupant != '.') {
            // try making space and retry
            move_pt2(adjusted, offset, input);
            target_cell_occupant = input->grid[(adjusted.y * input->width) + adjusted.x];
        }

        if (target_cell_occupant == '.') {
            // Move in
            input->grid[(adjusted.y * input->width) + adjusted.x] = input->grid[(to_move.y * input->width) + to_move.x];
            input->grid[(to_move.y * input->width) + to_move.x] = '.';

            return adjusted;
        } else {
            // Couldn't move
            return to_move;
        }
    }
}

void part2() {
    Input input = load_input(1);

    show_grid(&input);
    for (int move_idx = 0; move_idx < input.instruction_count; move_idx++) {
        Point offset = { 0 };

        switch (input.instructions[move_idx]) {
        case '^':
            offset.y -= 1;
            break;
        case '>':
            offset.x += 1;
            break;
        case '<':
            offset.x -= 1;
            break;
        case 'v':
            offset.y += 1;
            break;
        }

        input.robot = move_pt2(input.robot, offset, &input);
        // show_grid(&input);
    }

    int gps = 0;

    for (int row = 0; row < input.height; row++) {
        for (int col = 0; col < input.width; col++){
            if (input.grid[(row * input.width) + col] == '[') {
                gps += (100 * row) + col;
            }
        }
    }

    // show_grid(&input);

    printf("Part 2 GPS value: %d\n", gps);
}

int main() {
    part1();
    part2();

    return 0;
}
