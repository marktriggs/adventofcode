#define _POSIX_C_SOURCE 200112L


#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <assert.h>
#include <stdint.h>
#include <math.h>


#include "lib.c"

typedef struct {
    long a_x;
    long a_y;
    long b_x;
    long b_y;

    long target_x;
    long target_y;
} Game;

typedef struct {
    Game games[512];
    int count;
} Input;


Input load_input() {
    FILE *fh = fopen("input_files/day13.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    Input result = { 0 };

    char buf[4096];
    for (;;) {
        if (!fgets(buf, sizeof(buf) - 1, fh)) {
            break;
        }

        Game *this_game = &result.games[result.count++];

        assert(sscanf(buf, "Button A: X+%ld, Y+%ld", &this_game->a_x, &this_game->a_y) == 2);

        assert(fgets(buf, sizeof(buf) - 1, fh));
        assert(sscanf(buf, "Button B: X+%ld, Y+%ld", &this_game->b_x, &this_game->b_y) == 2);

        assert(fgets(buf, sizeof(buf) - 1, fh));
        assert(sscanf(buf, "Prize: X=%ld, Y=%ld", &this_game->target_x, &this_game->target_y) == 2);

        if (!fgets(buf, sizeof(buf) - 1, fh)) {
            break;
        }
    }

    return result;
}

typedef struct {
    long current_x;
    long current_y;

    long a_presses;
    long b_presses;
} GameState;


long max(long a, long b){
    return (a > b) ? a : b;
}

long solve(Game *game) {
    long lowest_score = LONG_MAX;

    for (long a = 0; a <= max((game->target_x / game->a_x), (game->target_y / game->a_y)); a++) {
        long remaining_x = game->target_x - (game->a_x * a);
        long remaining_y = game->target_y - (game->a_y * a);

        if ((remaining_x >= 0 && remaining_y >= 0) &&
            ((remaining_x % game->b_x) == 0 && (remaining_y % game->b_y) == 0) &&
            (remaining_x / game->b_x) == (remaining_y / game->b_y)) {
            long b = (remaining_x / game->b_x);

            if ((a * 3 + b) < lowest_score) {
                lowest_score = (a * 3 + b);
            }
        }
    }

    return lowest_score;
}

long solve_maxima(Game *game) {
    FILE *fp;

    char cmd[4096];
    char buffer[4096];

    sprintf(cmd, "maxima --very-quiet --batch-string='printf(false, \"~a\", linsolve([%ld*a+%ld*b=%ld,%ld*a+%ld*b=%ld], [a, b]));'",
            game->a_x, game->b_x, game->target_x,
            game->a_y, game->b_y, game->target_y);

    fp = popen(cmd, "r");
    assert(fp);

    long best = LONG_MAX;

    while (fgets(buffer, sizeof(buffer), fp) != NULL) {
        char *start = buffer;
        while (*start == ' ') {
            start++;
        }

        if (strncmp(start, "[a = ", strlen("[a = ")) != 0) {
            continue;
        }

        start += strlen("[a = ");

        int offset = 0;
        long a = (long)readint(start, &offset);
        start += offset;

        if (strncmp(start, ",b = ", strlen(",b = ")) != 0) {
            continue;
        }

        start += strlen(",b = ");

        offset = 0;
        long b = (long)readint(start, &offset);
        start += offset;

        if (*start != ']') {
            continue;
        }

        assert((a * game->a_x) + (b * game->b_x) == game->target_x);
        assert((a * game->a_y) + (b * game->b_y) == game->target_y);

        long result = a * 3 + b;

        if (result < best) {
            best = result;
        }
    }

    pclose(fp);

    return best;
}


void part1() {
    Input input = load_input();

    long total = 0;

    for (int game_idx = 0; game_idx < input.count; game_idx++) {
        Game *game = &input.games[game_idx];

        long lowest = solve(game);

        if (lowest != LONG_MAX) {
            total += lowest;
        }
    }

    printf("Part 1 total: %ld\n", total);
    fflush(stdout);
}

void part2_maxima() {
    Input input = load_input();

    long total = 0;

    for (int game_idx = 0; game_idx < input.count; game_idx++) {
        Game *game = &input.games[game_idx];

        game->target_x += 10000000000000L;
        game->target_y += 10000000000000L;

        long result = solve_maxima(game);

        if (result != LONG_MAX) {
            printf("Tokens for game %d: %ld\n", game_idx, result);
            total += result;
        }
    }

    printf("Part 2 total: %ld\n", total);
}

typedef struct {
  double a;
  double b;
  double c;
  double d;
} matrix2x2;

typedef struct {
  double a;
  double b;
} matrix2x1;

matrix2x1 matrix_multiply(matrix2x2 a, matrix2x1 b) {
  matrix2x1 result;

  result.a = (a.a * b.a) + (a.b * b.b);
  result.b = (a.c * b.a) + (a.d * b.b);

  return result;
}

matrix2x2 matrix_invert(matrix2x2 m) {
  double determinant = (m.a * m.d) - (m.b * m.c);

  matrix2x2 result;

  result.a = m.d / determinant;
  result.b = -m.b / determinant;
  result.c = -m.c / determinant;
  result.d = m.a / determinant;

  return result;
}


matrix2x1 solve_pt2_matrix(matrix2x2 coefficients, matrix2x1 targets) {
  return matrix_multiply(matrix_invert(coefficients), targets);
}


void part2() {
    Input input = load_input();

    long total = 0;

    for (int game_idx = 0; game_idx < input.count; game_idx++) {
        Game *game = &input.games[game_idx];

        game->target_x += 10000000000000L;
        game->target_y += 10000000000000L;

        matrix2x2 coefficients;
        coefficients.a = (double)game->a_x;
        coefficients.b = (double)game->b_x;
        coefficients.c = (double)game->a_y;
        coefficients.d = (double)game->b_y;

        matrix2x1 targets;

        targets.a = (double) game->target_x;
        targets.b = (double) game->target_y;

        matrix2x1 result = solve_pt2_matrix(coefficients, targets);

        // Still somewhat confused why ceil isn't the right answer here...
        long a_possibilities[2] = { (long)(floor(result.a)), (long)(ceil(result.a)) };
        long b_possibilities[2] = { (long)(floor(result.b)), (long)(ceil(result.b)) };

        int solved = 0;
        for (int a_idx = 0; !solved && a_idx < 2; a_idx++) {
            for (int b_idx = 0; !solved && b_idx < 2; b_idx++) {
                long x_total = (((long)coefficients.a * a_possibilities[a_idx]) + ((long)coefficients.b * b_possibilities[b_idx]));
                long y_total = ((long)coefficients.c * a_possibilities[a_idx]) + ((long)coefficients.d * b_possibilities[b_idx]);

                if ((x_total == game->target_x) && (y_total == game->target_y)) {
                    long score = (long)((3 * a_possibilities[a_idx]) + b_possibilities[b_idx]);
                    total += score;
                    solved = 1;
                } else {
                    // printf("%ld - %ld vs %ld - %ld\n", x_total, y_total, game->target_x, game->target_y);
                }
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
