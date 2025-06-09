#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#include "lib.c"

typedef struct {
    size_t len;
    char *bytes;
} str;


void free_str(str s) {
    free(s.bytes);
}

str load_input() {
    FILE *fh = fopen("input_files/day3.txt", "r");

    fseek(fh, 0L, SEEK_END);
    size_t size = (size_t)ftell(fh);
    rewind(fh);

    char *result = malloc(size);

    size_t read_bytes = fread(result, 1, size, fh);

    assert(read_bytes == size);

    fclose(fh);

    return (str){ .bytes = result, .len = size };
}

int read_constant(char *constant, str input, size_t offset) {
    size_t constant_len = strlen(constant);

    if ((offset + constant_len <= input.len)) {
        return memcmp(constant, input.bytes + offset, constant_len) == 0;
    } else {
        return 0;
    }
}

typedef struct {
    int success;
    int value;
    size_t end_position;
    int eof_hit;
} ParsedInt;

ParsedInt read_int(str s, size_t offset) {
    int started_read = 0;

    ParsedInt result = { 0 };

    size_t i;
    for (i = offset; i < s.len && s.bytes[i] >= '0' && s.bytes[i] <= '9'; i++) {
        started_read = 1;

        result.value *= 10;
        result.value += s.bytes[i] - '0';
    }

    result.success = started_read;
    result.end_position = i;

    return result;
}


void part1() {
    str input = load_input();

    int result = 0;
    size_t offset = 0;

    while (offset < input.len) {
        if (!read_constant("mul(", input, offset)) {
            offset++;
            continue;
        }

        size_t provisional_offset = offset + strlen("mul(");

        ParsedInt a = read_int(input, provisional_offset);
        if (!a.success) { goto not_a_mul; }
        provisional_offset = a.end_position;

        if (!read_constant(",", input, provisional_offset)) { goto not_a_mul; }
        provisional_offset++;

        ParsedInt b = read_int(input, provisional_offset);
        if (!b.success) { goto not_a_mul; }
        provisional_offset = b.end_position;

        if (!read_constant(")", input, provisional_offset)) { goto not_a_mul; }
        provisional_offset++;

        result += (a.value * b.value);

        offset = provisional_offset;
        continue;

    not_a_mul:
        offset++;
    }

    free_str(input);

    printf("Part 1 result: %d\n", result);
}

void part2() {
    str input = load_input();

    int result = 0;
    size_t offset = 0;

    int mul_enabled = 1;

    while (offset < input.len) {
        if (read_constant("do()", input, offset)) {
            offset += strlen("do()");
            mul_enabled = 1;
            continue;
        }

        if (read_constant("don't()", input, offset)) {
            offset += strlen("don't()");
            mul_enabled = 0;
            continue;
        }

        if (!read_constant("mul(", input, offset)) { goto not_a_mul; }
        size_t provisional_offset = offset + strlen("mul(");

        ParsedInt a = read_int(input, provisional_offset);
        if (!a.success) { goto not_a_mul; }
        provisional_offset = a.end_position;

        if (!read_constant(",", input, provisional_offset)) { goto not_a_mul; }
        provisional_offset++;

        ParsedInt b = read_int(input, provisional_offset);
        if (!b.success) { goto not_a_mul; }
        provisional_offset = b.end_position;

        if (!read_constant(")", input, provisional_offset)) { goto not_a_mul; }
        provisional_offset++;

        if (mul_enabled) {
            result += (a.value * b.value);
        }

        offset = provisional_offset;
        continue;

    not_a_mul:
        offset++;
    }

    free_str(input);

    printf("Part 2 result: %d\n", result);
}

int main() {
    part1();
    part2();

    return 0;
}
