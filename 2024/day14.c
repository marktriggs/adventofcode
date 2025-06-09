// FFMPEG_OUTPUT=1 ./day14 | pv | ffmpeg -vcodec ppm -f image2pipe -framerate 60 -i - out.mp4

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <assert.h>
#include <stdint.h>
#include <sys/stat.h>
#include <sys/types.h>


#include "lib.c"

typedef struct {
    int x;
    int y;

    int x_velocity;
    int y_velocity;
} Robot;

#define MAX_ROBOTS 1024

typedef struct {
    Robot robots[MAX_ROBOTS];
    int count;
} Robots;

Robots load_input() {
    FILE *fh = fopen("input_files/day14.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    Robots result = { 0 };

    char buf[4096];

    while (fgets(buf, sizeof(buf) - 1, fh)) {
        Robot *robot = &result.robots[result.count++];

        assert(sscanf(buf, "p=%d,%d v=%d,%d", &(robot->x), &(robot->y), &(robot->x_velocity), &(robot->y_velocity)) == 4);
    }

    fclose(fh);

    return result;
}


void part1() {
    Robots robots = load_input();

    int grid_width = 101;
    int grid_height = 103;

    for (int time = 0; time < 100; time++) {
        for (int robot_idx = 0; robot_idx < robots.count; robot_idx++) {
            Robot *robot = &robots.robots[robot_idx];

            robot->x += robot->x_velocity;
            robot->y += robot->y_velocity;

            robot->x = ((robot->x % grid_width) + grid_width) % grid_width;
            robot->y = ((robot->y % grid_height) + grid_height) % grid_height;
        }
    }

    int quadrant_counts[2][2] = { 0 };

    for (int robot_idx = 0; robot_idx < robots.count; robot_idx++) {
        Robot *robot = &robots.robots[robot_idx];

        if (robot->x == (grid_width / 2) || robot->y == (grid_height / 2)) {
            // On the centre line
        } else {
            quadrant_counts[robot->x > (grid_width / 2)][robot->y > (grid_height / 2)]++;
        }
    }

    printf("Part 1: Safety factor %d\n", quadrant_counts[0][0] * quadrant_counts[0][1] * quadrant_counts[1][0] * quadrant_counts[1][1]);
}

void print_grid(Robots *robots, int grid_width, int grid_height) {
    char output[grid_width * grid_height];

    for (int i = 0; i < alen(output); i++) {
        output[i] = '.';
    }

    for (int robot_idx = 0; robot_idx < robots->count; robot_idx++) {
        Robot *robot = &robots->robots[robot_idx];
        output[(robot->y * grid_width) + robot->x] = '#';
    }

    int pixel_size = 4;

    int pixel_width = grid_width * pixel_size;
    int pixel_height = grid_height * pixel_size;

    fprintf(stdout, "P6\n");
    fprintf(stdout, "%d\n", pixel_width);
    fprintf(stdout, "%d\n", pixel_height);
    fprintf(stdout, "255\n");

    for (int row = 0; row < grid_height; row++) {
        for (int px_row = 0; px_row < pixel_size; px_row++) {
            for (int col = 0; col < grid_width; col++) {
                for (int px_col = 0; px_col < pixel_size; px_col++) {
                    int val = (output[(row * grid_width) + col] == '.' ? 255 : 0);
                    fprintf(stdout, "%c%c%c", val, val, val);
                }
            }
        }
    }
}


void print_metrics(int time, Robots *robots, int grid_width, int grid_height) {
    char output[grid_width * grid_height];

    for (int i = 0; i < alen(output); i++) {
        output[i] = '.';
    }

    for (int robot_idx = 0; robot_idx < robots->count; robot_idx++) {
        Robot *robot = &robots->robots[robot_idx];
        output[(robot->y * grid_width) + robot->x] = '#';
    }

    int current_run = 0;
    int best_run = 0;

    for (int row = 0; row < grid_height; row++) {
        current_run = 0;
        for (int col = 0; col < grid_width; col++) {
            if (output[(row * grid_width) + col] == '#') {
                current_run++;
            } else {
                if (current_run > best_run) {
                    best_run = current_run;
                    current_run = 0;
                }
            }
        }
    }

    printf("Time %d Best run: %d\n", time, best_run);
}

void part2(int do_print) {
    Robots robots = load_input();

    int grid_width = 101;
    int grid_height = 103;

    for (int time = 0; time < 2000000; time++) {
        for (int robot_idx = 0; robot_idx < robots.count; robot_idx++) {
            Robot *robot = &robots.robots[robot_idx];

            robot->x += robot->x_velocity;
            robot->y += robot->y_velocity;

            robot->x = ((robot->x % grid_width) + grid_width) % grid_width;
            robot->y = ((robot->y % grid_height) + grid_height) % grid_height;
        }

        if (do_print) {
            print_grid(&robots, grid_width, grid_height);
        } else {
            print_metrics(time, &robots, grid_width, grid_height);
        }
    }

    int quadrant_counts[2][2] = { 0 };

    for (int robot_idx = 0; robot_idx < robots.count; robot_idx++) {
        Robot *robot = &robots.robots[robot_idx];

        if (robot->x == (grid_width / 2) || robot->y == (grid_height / 2)) {
            // On the centre line
        } else {
            quadrant_counts[robot->x > (grid_width / 2)][robot->y > (grid_height / 2)]++;
        }
    }

    if (!do_print) {
        printf("Nothing printed because FFMPEG_OUTPUT isn't set");
    }
}


int main() {
    int do_print = (getenv("FFMPEG_OUTPUT") != NULL);

    if (!do_print) {
        part1();
    }

    part2(do_print);

    return 0;
}
