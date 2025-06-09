#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#include "lib.c"

typedef struct {
    char bytes[32768];
    int width;
    int height;
} Grid;


Grid load_input() {
    FILE *fh = fopen("input_files/day4.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    fseek(fh, 0L, SEEK_END);
    size_t size = (size_t)ftell(fh);
    rewind(fh);

    Grid result = { 0 };

    size_t read_bytes = fread(result.bytes, 1, size, fh);

    assert(read_bytes == size);

    fclose(fh);

    while (result.bytes[result.width] != '\n') {
        result.width++;
    }

    for (int i = 0; i < (int)size; i++) {
        if (result.bytes[i] == '\n') {
            result.height++;
        }
    }

    return result;
}


int matches(Grid *input, int from_x, int from_y, int x_direction, int y_direction, char *target) {
    int len = (int)strlen(target);

    for (int i = 0; i < len; i++) {
        int x = from_x + x_direction * i;
        int y = from_y + y_direction * i;

        if (x < 0 || x >= input->width) { return 0; }
        if (y < 0 || y >= input->height) { return 0; }

        /* Skipping an extra 1 here for the \n chars */
        if (input->bytes[(y * (input->width + 1)) + x] != target[i]) {
            return 0;
        }
    }

    return 1;
}

void part1() {
    Grid input = load_input();

    int result = 0;

    for (int y = 0; y < input.height; y++) {
        for (int x = 0; x < input.width; x++) {
            // horizontals
            if (matches(&input, x, y, 1, 0, "XMAS") || matches(&input, x, y, 1, 0, "SAMX")) {
                result++;
            }

            // verticals
            if (matches(&input, x, y, 0, 1, "XMAS") || matches(&input, x, y, 0, 1, "SAMX")) {
                result++;
            }

            // \ diagonals
            if (matches(&input, x, y, 1, 1, "XMAS") || matches(&input, x, y, 1, 1, "SAMX")) {
                result++;
            }

            // / diagonals
            if (matches(&input, x, y, -1, 1, "XMAS") || matches(&input, x, y, -1, 1, "SAMX")) {
                result++;
            }
        }
    }

    printf("Part 1 match count: %d\n", result);
}

void part2() {
    Grid input = load_input();

    int result = 0;

    for (int y = 0; y < input.height; y++) {
        for (int x = 0; x < input.width; x++) {
            if ((matches(&input, x, y, 1, 1, "MAS") || matches(&input, x, y, 1, 1, "SAM")) &&
                (matches(&input, x + 2, y, -1, 1, "MAS") || matches(&input, x + 2, y, -1, 1, "SAM"))) {
                result++;
            }
        }
    }

    printf("Part 2 match count: %d\n", result);
}


int main() {
    part1();
    part2();

    return 0;
}
