#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <assert.h>
#include <stdint.h>

#include "lib.c"

typedef struct {
    int grid[32768];
    int width;
    int height;
} Grid;


Grid load_input() {
    FILE *fh = fopen("input_files/day12.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    Grid result = { 0 };

    char buf[4096];

    int row = -1;

    int pos = 0;
    while (fgets(buf, sizeof(buf) - 1, fh)) {
        row++;

        for (int col = 0; buf[col] != '\n'; col++) {
            result.width = col;

            result.grid[pos++] = buf[col];
            assert((size_t)pos < sizeof result.grid);
        }
    }

    result.width++;
    result.height = row + 1;

    fclose(fh);

    return result;
}

int grid_elt(Grid *grid, int row, int col) {
    if ((row >= 0 && row < grid->height) && (col >= 0 && col < grid->width)) {
        return grid->grid[(row * grid->width) + col];
    } else {
        return -1;
    }
}

void grid_set(Grid *grid, int row, int col, int value) {
    assert ((row >= 0 && row < grid->height) && (col >= 0 && col < grid->width));
    grid->grid[(row * grid->width) + col] = value;
}


void grid_print(Grid *grid) {
    for (int row = 0; row < grid->height; row++) {
        for (int col = 0; col < grid->width; col++) {
            if (grid_elt(grid, row, col) == 0) {
                printf("[  ]");
            } else {
                printf("[%02d]", grid_elt(grid, row, col));
            }
        }
        printf("\n");
    }
}


typedef struct {
    Grid grid;
    int next_group_id;
} GridGroups;

GridGroups find_grid_groups(Grid *input) {
    Grid groups = (Grid){
        .width = input->width,
        .height = input->height,
        .grid = { 0 },
    };

    int next_group_id = 1;

    for (int row = 0; row < input->height; row++) {
        for (int col = 0; col < input->width; col++) {
            assert(grid_elt(&groups, row, col) == 0);

            int this_value = grid_elt(input, row, col);

            if (this_value == 0) {
                // Skip empty cells
                grid_set(&groups, row, col, 0);
                continue;
            }

            int left_group = -1;
            int above_group = -1;

            if (grid_elt(input, row, col - 1) == this_value) {
                // We're in the same group as the guy before us
                left_group = grid_elt(&groups, row, col - 1);
                assert(left_group > 0);
            }

            if (grid_elt(input, row - 1, col) == this_value) {
                // We're in the same group as the guy above us
                above_group = grid_elt(&groups, row - 1, col);
                assert(above_group > 0);
            }

            if (left_group < 0 && above_group < 0) {
                // We're our own thing
                grid_set(&groups, row, col, next_group_id++);
            } else if (left_group > 0 && above_group < 0) {
                // Same as left group
                grid_set(&groups, row, col, left_group);
            } else if (left_group < 0 && above_group > 0) {
                // Same as above group
                grid_set(&groups, row, col, above_group);
            } else if (left_group == above_group) {
                // We're part of a happy family
                grid_set(&groups, row, col, above_group);
            } else {
                // Conflict!  Both groups claim us.
                grid_set(&groups, row, col, above_group);

                // Resolve the conflict by combining the two groups.  We're the missing link!
                for (int group_row = 0; group_row < groups.height; group_row++) {
                    for (int group_col = 0; group_col < groups.width; group_col++) {
                        if (grid_elt(&groups, group_row, group_col) == left_group) {
                            grid_set(&groups, group_row, group_col, above_group);
                        }
                    }
                }
            }
        }
    }

    return (GridGroups) {
        .grid = groups,
        .next_group_id = next_group_id,
    };
}


void part1() {
    Grid input = load_input();

    GridGroups grid_groups = find_grid_groups(&input);

    Grid groups = grid_groups.grid;
    int next_group_id = grid_groups.next_group_id;

    // Produce group perimiters
    int group_perimiter[next_group_id];
    {
        bzero(&group_perimiter, sizeof(group_perimiter));

        for (int row = 0; row < input.height; row++) {
            for (int col = 0; col < input.width; col++) {
                int group = grid_elt(&groups, row, col);
                assert(group > 0);

                int perimiter = 0;

                if (grid_elt(&groups, row - 1, col) != group) { perimiter++; }
                if (grid_elt(&groups, row, col - 1) != group) { perimiter++; }
                if (grid_elt(&groups, row + 1, col) != group) { perimiter++; }
                if (grid_elt(&groups, row, col + 1) != group) { perimiter++; }

                group_perimiter[group] += perimiter;
            }
        }

        // for (int group = 1; group < next_group_id; group++) {
        //     printf("Perimiter for group %d: %d\n", group, group_perimiter[group]);
        // }
    }


    // Produce group area
    int group_area[next_group_id];
    {
        bzero(&group_area, sizeof(group_area));

        for (int row = 0; row < input.height; row++) {
            for (int col = 0; col < input.width; col++) {
                int group = grid_elt(&groups, row, col);
                assert(group > 0);

                group_area[group]++;
            }
        }

        // for (int group = 1; group < next_group_id; group++) {
        //     printf("Area for group %d: %d\n", group, group_area[group]);
        // }
    }

    int total_cost = 0;
    for (int group = 1; group < next_group_id; group++) {
        total_cost += (group_area[group] * group_perimiter[group]);
    }

    printf("Part 1 total cost: %d\n", total_cost);
}


void part2() {
    Grid input = load_input();

    GridGroups grid_groups = find_grid_groups(&input);

    Grid groups = grid_groups.grid;
    int next_group_id = grid_groups.next_group_id;

    // Produce group sides
    int group_sides[next_group_id];
    {
        bzero(&group_sides, sizeof(group_sides));

        for (int group = 1; group < next_group_id; group++) {
            Grid horizontal_upper_sides = {
                .width = input.width + 2,
                .height = input.height + 2,
                .grid = { 0 },
            };

            Grid horizontal_lower_sides = {
                .width = input.width + 2,
                .height = input.height + 2,
                .grid = { 0 },
            };

            Grid vertical_left_sides = {
                .width = input.width + 2,
                .height = input.height + 2,
                .grid = { 0 },
            };

            Grid vertical_right_sides = {
                .width = input.width + 2,
                .height = input.height + 2,
                .grid = { 0 },
            };

            {
                for (int row = 0; row < input.height; row++) {
                    for (int col = 0; col < input.width; col++) {
                        if (grid_elt(&groups, row, col) != group) {
                            continue;
                        }

                        int side_row = row + 1;
                        int side_col = col + 1;

                        if (grid_elt(&groups, row - 1, col) != group) { grid_set(&horizontal_upper_sides, side_row - 1, side_col, '-'); }
                        if (grid_elt(&groups, row + 1, col) != group) { grid_set(&horizontal_lower_sides, side_row + 1, side_col, '-'); }
                        if (grid_elt(&groups, row, col - 1) != group) { grid_set(&vertical_left_sides, side_row, side_col - 1, '|'); }
                        if (grid_elt(&groups, row, col + 1) != group) { grid_set(&vertical_right_sides, side_row, side_col + 1, '|'); }
                    }
                }
            }

            GridGroups grouped_horizontal_upper_sides = find_grid_groups(&horizontal_upper_sides);
            GridGroups grouped_horizontal_lower_sides = find_grid_groups(&horizontal_lower_sides);
            GridGroups grouped_vertical_left_sides = find_grid_groups(&vertical_left_sides);
            GridGroups grouped_vertical_right_sides = find_grid_groups(&vertical_right_sides);

            group_sides[group] += grouped_horizontal_upper_sides.next_group_id - 1;
            group_sides[group] += grouped_horizontal_lower_sides.next_group_id - 1;
            group_sides[group] += grouped_vertical_left_sides.next_group_id - 1;
            group_sides[group] += grouped_vertical_right_sides.next_group_id - 1;
        }
    }

    // Produce group area
    int group_area[next_group_id];
    {
        bzero(&group_area, sizeof(group_area));

        for (int row = 0; row < input.height; row++) {
            for (int col = 0; col < input.width; col++) {
                int group = grid_elt(&groups, row, col);
                assert(group > 0);

                group_area[group]++;
            }
        }
    }

    int total_cost = 0;
    for (int group = 1; group < next_group_id; group++) {
        total_cost += (group_area[group] * group_sides[group]);
    }

    printf("Part 2 total cost: %d\n", total_cost);
}

int main() {
    part1();
    part2();

    return 0;
}
