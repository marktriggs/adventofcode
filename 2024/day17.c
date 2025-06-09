#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <assert.h>
#include <stdint.h>
#include <sys/stat.h>
#include <sys/types.h>

#include "lib.c"

#define MAX_PROGRAM_SIZE 4096

typedef struct {
    uint64_t instructions[MAX_PROGRAM_SIZE];
    uint64_t instruction_count;
    uint64_t registers[3];
    uint64_t output_buffer[MAX_PROGRAM_SIZE];
    int output_count;
    uint64_t ip;
} Computer;

typedef enum {
    a = 0,
    b = 1,
    c = 2,
} Register;

typedef enum {
    adv = 0,
    bxl = 1,
    bst = 2,
    jnz = 3,
    bxc = 4,
    out = 5,
    bdv = 6,
    cdv = 7,
    instruction_count,
} Instruction;


uint64_t combo_operand_value(Computer *computer, uint64_t operand) {
    switch (operand) {
    case 0:
    case 1:
    case 2:
    case 3:
        return operand;
    case 4:
    case 5:
    case 6:
        return computer->registers[operand - 4];
    case 7:
        // reserved
        assert(0);
        break;
    }

    printf("Weird operand: %ld\n", operand);
    assert(0);
}

int interpret(Computer *computer) {
    if (computer->ip >= computer->instruction_count) {
        // Halt
        return 0;
    }

    switch (computer->instructions[computer->ip++]) {
    case adv:
    {
        uint64_t numerator = computer->registers[a];
        uint64_t operand = computer->instructions[computer->ip++];

        computer->registers[a] = numerator >> combo_operand_value(computer, operand);

        break;
    }
    case bxl:
    {
        computer->registers[b] = computer->registers[b] ^ computer->instructions[computer->ip++];
        break;
    }
    case bst:
    {
        uint64_t operand = combo_operand_value(computer, computer->instructions[computer->ip++]);
        computer->registers[b] = operand % 8;
        break;
    }
    case jnz:
    {
        if (computer->registers[a] != 0) {
            computer->ip = computer->instructions[computer->ip++];
        } else {
            // Discard operand?
            computer->ip++;
        }

        break;
    }
    case bxc:
    {
        computer->registers[b] = computer->registers[b] ^ computer->registers[c];
        computer->ip++;

        break;
    }
    case out:
    {
        computer->output_buffer[computer->output_count++] = combo_operand_value(computer, computer->instructions[computer->ip++]) % 8;
        break;
    }
    case bdv:
    {
        uint64_t numerator = computer->registers[a];
        uint64_t operand = computer->instructions[computer->ip++];

        computer->registers[b] = numerator >> combo_operand_value(computer, operand);

        break;
    }
    case cdv:
    {
        uint64_t numerator = computer->registers[a];
        uint64_t operand = computer->instructions[computer->ip++];

        computer->registers[c] = numerator >> combo_operand_value(computer, operand);

        break;
    }
    }

    // Continue
    return 1;
}

void part1() {
    Computer computer = { 0 };
    uint64_t input[] = { 2, 4, 1, 7, 7, 5, 1, 7, 0, 3, 4, 1, 5, 5, 3, 0 };

    for (int i = 0; i < alen(input); i++) {
        computer.instructions[i] = input[i];
    }

    computer.instruction_count = alen(input);

    computer.registers[a] = 66752888;

    // printf("Initial state:\n");
    // printf("IP: %d\n", computer.ip);
    // printf("A: %d\n", computer.registers[a]);
    // printf("B: %d\n", computer.registers[b]);
    // printf("C: %d\n", computer.registers[c]);

    while (interpret(&computer)) {
        // printf("IP: %d\n", computer.ip);
        // printf("A: %d\n", computer.registers[a]);
        // printf("B: %d\n", computer.registers[b]);
        // printf("C: %d\n", computer.registers[c]);
    }

    for (int i = 0; i < computer.output_count; i++) {
        if (i > 0) {
            printf(",");
        }
        printf("%ld", computer.output_buffer[i]);
    }

    printf("\n");
}

// Worked this out by transcribing the ASM program and simplifying to express
// the print statement in terms of A
uint64_t part2_print_value(uint64_t a) {
    return ((a % 8) ^ (a / 1 >> (7 - (a % 8)))) % 8;
}


void solve(uint64_t *input, int input_len, int input_idx, uint64_t solved_a, uint64_t min_next_a) {
    printf("min: %ld for input idx %d\n", min_next_a, input_idx);

    if (input_idx < 0) {
        printf("Solution: %ld\n", solved_a);
    }

    uint64_t target_value = input[input_idx];

    for (int i = 0; i < 8; i++) {
        uint64_t candidate = min_next_a + (uint64_t)i;

        if (part2_print_value(candidate) == target_value) {
            printf("%ld\n", candidate);
            solve(input, input_len, input_idx - 1, candidate, candidate * 8);
        }
    }
}

void part2() {
    uint64_t input[] = { 2, 4, 1, 7, 7, 5, 1, 7, 0, 3, 4, 1, 5, 5, 3, 0 };
    solve(input, alen(input), alen(input) - 1, 0, 1);
}

int main() {
    part1();
    part2();

    return 0;
}
