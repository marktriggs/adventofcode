#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <assert.h>
#include <stdint.h>

#include "lib.c"

#define MAX_TERMS 32
#define MAX_EQUATIONS 1024

typedef struct {
    uint64_t terms[MAX_TERMS];
    int term_count;
    uint64_t target_value;
} Equation;

typedef struct {
    Equation equations[MAX_EQUATIONS];
    int count;
} Input;

Input load_input() {
    FILE *fh = fopen("input_files/day7.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    Input result = { 0 };

    char buf[4096];

    while (fgets(buf, sizeof(buf) - 1, fh)) {
        int offset = 0;

        Equation *equation = &result.equations[result.count++];
        assert(result.count < MAX_EQUATIONS);

        equation->target_value = (uint64_t)readint(buf, &offset);

        offset += 2;

        while (buf[offset] != '\0') {
            equation->terms[equation->term_count++] = (uint64_t)readint(buf, &offset);
            assert(equation->term_count < MAX_TERMS);
            offset++;
        }

    }

    fclose(fh);

    return result;
}


int solvable_part1(Equation *equation, uint64_t current_value, int current_offset) {
    if (current_offset == equation->term_count) {
        return current_value == equation->target_value;
    }

    assert((current_value + equation->terms[current_offset]) >= current_value);
    assert((current_value * equation->terms[current_offset]) >= current_value);

    return solvable_part1(equation, current_value + equation->terms[current_offset], current_offset + 1) ||
        solvable_part1(equation, current_value * equation->terms[current_offset], current_offset + 1);
}

void part1() {
    Input input = load_input();

    uint64_t result = 0;

    for (int equation_idx = 0; equation_idx < input.count; equation_idx++) {
        Equation *equation = &input.equations[equation_idx];

        if (solvable_part1(equation, equation->terms[0], 1)) {
            result += (uint64_t)equation->target_value;
        }
    }

    printf("Part 1 solvable sum: %ld\n", result);
}

uint64_t concatenate(uint64_t a, uint64_t b) {
    int log = intlog10(b);
    for (int i = 0; i < log; i++) {
        a *= 10;
    }

    uint64_t result = a + b;

    assert(result > a);

    return result;
}

int solvable_part2(Equation *equation, uint64_t current_value, int current_offset) {
    if (current_offset == equation->term_count) {
        return current_value == equation->target_value;
    }

    assert((current_value + equation->terms[current_offset]) >= current_value);
    assert((current_value * equation->terms[current_offset]) >= current_value);

    return (solvable_part2(equation, current_value + equation->terms[current_offset], current_offset + 1) ||
            solvable_part2(equation, current_value * equation->terms[current_offset], current_offset + 1) ||
            solvable_part2(equation,
                           concatenate(current_value, equation->terms[current_offset]),
                           current_offset + 1));
}


void part2() {
    Input input = load_input();

    uint64_t result = 0;

    for (int equation_idx = 0; equation_idx < input.count; equation_idx++) {
        Equation *equation = &input.equations[equation_idx];

        if (solvable_part2(equation, equation->terms[0], 1)) {
            result += (uint64_t)equation->target_value;
        }
    }

    printf("Part 2 solvable sum: %ld\n", result);
}

int main() {
    part1();
    part2();

    return 0;
}
