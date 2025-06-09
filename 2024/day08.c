#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <assert.h>
#include <stdint.h>

#include "lib.c"

typedef struct {
    int row;
    int col;
} Point;

typedef struct {
    Point locations[128];
    int count;
} PointList;

typedef struct {
    PointList frequency[256];
    int width;
    int height;
} Input;


Input load_input() {
    FILE *fh = fopen("input_files/day8.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    Input result = { 0 };

    char buf[4096];

    int row = -1;

    while (fgets(buf, sizeof(buf) - 1, fh)) {
        row++;

        for (int col = 0; buf[col] != '\n'; col++) {
            result.width = col;
            if (buf[col] == '.') {
                continue;
            }

            PointList *frequency_points = &result.frequency[(int)buf[col]];

            frequency_points->locations[frequency_points->count++] = (Point) { .row = row, .col = col };
            assert(frequency_points->count < alen(frequency_points->locations));

        }
    }

    result.width++;
    result.height = row + 1;

    fclose(fh);

    return result;
}


Point manhattan_distance(Point a, Point b) {
    return (Point) {
        .row = a.row - b.row,
        .col = a.col - b.col,
    };
}

Point point_negate(Point p) {
    return (Point) {
        .row = -p.row,
        .col = -p.col,
    };
}

Point point_add(Point a, Point b) {
    return (Point) {
        .row = a.row + b.row,
        .col = a.col + b.col,
    };
}

int in_range(Point p, int width, int height) {
    return ((p.row >= 0 && p.row < width) && (p.col >= 0 && p.col < height));
}

void part1() {
    Input input = load_input();

    int unique_locations[input.width * input.height];
    bzero(unique_locations, sizeof unique_locations);

    for (int antenna = 0; antenna < 256; antenna++) {
        if (input.frequency[antenna].count > 1) {
            for (int i = 0; i < input.frequency[antenna].count - 1; i++) {
                for (int j = i + 1; j < input.frequency[antenna].count; j++) {
                    Point antenna_a = input.frequency[antenna].locations[i];
                    Point antenna_b = input.frequency[antenna].locations[j];

                    Point distance = manhattan_distance(antenna_b, antenna_a);

                    Point antinode_a = point_add(antenna_a, point_negate(distance));
                    Point antinode_b = point_add(antenna_b, distance);

                    if (in_range(antinode_a, input.width, input.height)) {
                        unique_locations[(antinode_a.row * input.width) + antinode_a.col] = 1;
                    }

                    if (in_range(antinode_b, input.width, input.height)) {
                        unique_locations[(antinode_b.row * input.width) + antinode_b.col] = 1;
                    }
                }
            }
        }
    }

    int result = 0;
    for (int i = 0; i < alen(unique_locations); i++) {
        if (unique_locations[i]) {
            result++;
        }
    }

    printf("Part 1 result: %d\n", result);
}

void part2() {
    Input input = load_input();

    int unique_locations[input.width * input.height];
    bzero(unique_locations, sizeof unique_locations);

    for (int antenna = 0; antenna < 256; antenna++) {
        if (input.frequency[antenna].count > 1) {
            for (int i = 0; i < input.frequency[antenna].count - 1; i++) {
                for (int j = i + 1; j < input.frequency[antenna].count; j++) {
                    Point antenna_a = input.frequency[antenna].locations[i];
                    Point antenna_b = input.frequency[antenna].locations[j];

                    Point distance = manhattan_distance(antenna_b, antenna_a);

                    Point antinode_a = antenna_a;
                    for (;;) {
                        if (in_range(antinode_a, input.width, input.height)) {
                            unique_locations[(antinode_a.row * input.width) + antinode_a.col] = 1;
                        } else {
                            break;
                        }

                        antinode_a = point_add(antinode_a, point_negate(distance));
                    }

                    Point antinode_b = antenna_b;
                    for (;;) {
                        if (in_range(antinode_b, input.width, input.height)) {
                            unique_locations[(antinode_b.row * input.width) + antinode_b.col] = 1;
                        } else {
                            break;
                        }

                        antinode_b = point_add(antinode_b, distance);
                    }
                }
            }
        }
    }

    int result = 0;
    for (int i = 0; i < alen(unique_locations); i++) {
        if (unique_locations[i]) {
            result++;
        }
    }

    printf("Part 2 result: %d\n", result);
}


int main() {
    part1();
    part2();

    return 0;
}
