// Precomputed panel moves using day21.rb

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <stdint.h>

#include "lib.c"


typedef struct {
    char direction;
    int magnitude;
} movement;

typedef struct {
    int count;
    movement movements[3];
} movements;

typedef struct {
    int count;
    movements options[2];
} movement_options;

static movement_options NUMBER_PANEL_MOVES[256][256] = { 0 };
static movement_options DIRECTION_PANEL_MOVES[256][256] = { 0 };


void initialise_movement_tables() {
    #include "day21_table.h"
}


unsigned int cache_key(movements desired_move, int depth) {
    unsigned int result = 0;

    // 5 bits per movement for a total possible of 15
    for (int i = 0; i < desired_move.count; i++) {
        result <<= 3;

        switch (desired_move.movements[i].direction) {
        case '^':
            result |= 0;
            break;
        case '<':
            result |= 1;
            break;
        case 'v':
            result |= 2;
            break;
        case '>':
            result |= 3;
            break;
        case 'A':
            result |= 4;
            break;
        }

        result <<= 2;
        result |= (unsigned int)desired_move.movements[i].magnitude;
    }

    // A further 5 bits for the depth (up to 25 in real life)
    result <<= 5;
    result |= (unsigned int) depth;

    return result;
}

static int64_t *CACHE;

int64_t lowest_directional_cost(movements desired_move, int depth) {
    unsigned int key = cache_key(desired_move, depth);

    if (CACHE[key] > 0) {
        return CACHE[key];
    }

    int64_t result = 0;

    if (depth == 0) {
        for (int i = 0; i < desired_move.count; i++) {
            result += desired_move.movements[i].magnitude;
        }
    } else {
        char current_cell = 'A';

        for (int i = 0; i < desired_move.count; i++) {
            int64_t best_cost = INT64_MAX;

            movement m = desired_move.movements[i];

            movement_options options = DIRECTION_PANEL_MOVES[(int)current_cell][(int)m.direction];

            for (int option_idx = 0; option_idx < options.count; option_idx++) {
                int64_t cost = lowest_directional_cost(options.options[option_idx], depth - 1);
                cost += (m.magnitude - 1);

                if (cost < best_cost) {
                    best_cost = cost;
                }
            }

            result += best_cost;
            current_cell = m.direction;
        }
    }

    CACHE[key] = result;

    return result;
}


int64_t lowest_cost(char *code, int directional_panel_count) {
    char current_cell = 'A';

    int64_t total_cost = 0;

    for (char *digit = code; *digit; digit++) {
        int64_t best_cost = INT64_MAX;

        movement_options options = NUMBER_PANEL_MOVES[(int)current_cell][(int)*digit];

        assert(options.count > 0);

        for (int option_idx = 0; option_idx < options.count; option_idx++) {
            int64_t cost = lowest_directional_cost(options.options[option_idx], directional_panel_count);

            if (cost < best_cost) {
                best_cost = cost;
            }
        }

        current_cell = *digit;
        total_cost += best_cost;
    }

    return total_cost;
}

void part1() {
    #include "input_files/day21.txt"

    int64_t total_complexity = 0;

    for (int code_idx = 0; code_idx < alen(codes_to_enter); code_idx++) {
        int64_t cost = lowest_cost(codes_to_enter[code_idx], 2);
        int64_t numeric = 0;

        for (char *c = codes_to_enter[code_idx]; *c; c++) {
            if (*c >= '0' && *c <= '9') {
                numeric *= 10;
                numeric += *c - '0';
            }
        }

        total_complexity += (cost * numeric);
    }

    printf("Part 1: Total complexity: %ld\n", total_complexity);
}

void part2() {
    #include "input_files/day21.txt"

    int64_t total_complexity = 0;

    for (int code_idx = 0; code_idx < alen(codes_to_enter); code_idx++) {
        int64_t cost = lowest_cost(codes_to_enter[code_idx], 25);
        int64_t numeric = 0;

        for (char *c = codes_to_enter[code_idx]; *c; c++) {
            if (*c >= '0' && *c <= '9') {
                numeric *= 10;
                numeric += *c - '0';
            }
        }

        total_complexity += (cost * numeric);
    }

    printf("Part 2: Total complexity: %ld\n", total_complexity);
}



int main() {
    initialise_movement_tables();

    // Enough space based on the bits used by cache_key (20 bits).
    //
    // Too lazy to use a proper hash table.  Let's just call it perfect hashing and
    // pretend we're virtuous.
    size_t cache_size = 1 << 20;

    CACHE = malloc(cache_size * sizeof(int64_t));

    memset(CACHE, 0, cache_size * sizeof(int64_t));
    part1();

    memset(CACHE, 0, cache_size * sizeof(int64_t));
    part2();
}
