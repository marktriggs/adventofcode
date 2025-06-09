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
} Input;


Input load_input() {
    FILE *fh = fopen("input_files/day10.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    Input result = { 0 };

    char buf[4096];

    int row = -1;

    int pos = 0;
    while (fgets(buf, sizeof(buf) - 1, fh)) {
        row++;

        for (int col = 0; buf[col] != '\n'; col++) {
            result.width = col;

            result.grid[pos++] = buf[col] - '0';
            assert((size_t)pos < sizeof result.grid);
        }
    }

    result.width++;
    result.height = row + 1;

    fclose(fh);

    return result;
}

uint64_t score_trailhead_p1(Input *input, int x, int y) {
    int this_value = input->grid[(y * input->width) + x];

    if (this_value == 9) {
        input->grid[(y * input->width) + x] = 0; // mark it off
        return 1;
    }

    uint64_t score = 0;

    for (int xoff = -1; xoff <= 1; xoff++) {
        for (int yoff = -1; yoff <= 1; yoff++) {
            // Only up/down/left/right
            if (abs(xoff + yoff) != 1) {
                continue;
            }

            int next_x = x + xoff;
            int next_y = y + yoff;

            if ((next_x >= 0 && next_x < input->width) &&
                (next_y >= 0 && next_y < input->height) &&
                input->grid[(next_y * input->width) + next_x] == (this_value + 1)) {
                score += score_trailhead_p1(input, next_x, next_y);
            }
        }
    }

    return score;
}

uint64_t score_trailhead_p2(Input *input, int x, int y) {
    int this_value = input->grid[(y * input->width) + x];

    if (this_value == 9) {
        return 1;
    }

    uint64_t score = 0;

    for (int xoff = -1; xoff <= 1; xoff++) {
        for (int yoff = -1; yoff <= 1; yoff++) {
            // Only up/down/left/right
            if (abs(xoff + yoff) != 1) {
                continue;
            }

            int next_x = x + xoff;
            int next_y = y + yoff;

            if ((next_x >= 0 && next_x < input->width) &&
                (next_y >= 0 && next_y < input->height) &&
                input->grid[(next_y * input->width) + next_x] == (this_value + 1)) {
                score += score_trailhead_p2(input, next_x, next_y);
            }
        }
    }

    return score;
}


void part1() {
    Input input = load_input();

    uint64_t total = 0;

    for (int y = 0; y < input.height; y++) {
        for (int x = 0; x < input.width; x++) {
            if (input.grid[(y * input.width) + x] == 0) {
                Input clone = input;

                total += score_trailhead_p1(&clone, x, y);
            }
        }
    }

    printf("Part 1 total: %ld\n", total);
}

void part2() {
    Input input = load_input();

    uint64_t total = 0;

    for (int y = 0; y < input.height; y++) {
        for (int x = 0; x < input.width; x++) {
            if (input.grid[(y * input.width) + x] == 0) {
                total += score_trailhead_p2(&input, x, y);
            }
        }
    }

    printf("Part 2 total: %ld\n", total);
}



int main() {
    part1();
    part2();

    return 0;
}
