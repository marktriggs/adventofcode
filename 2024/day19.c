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

typedef struct Trie {
    struct Trie *links[256];
    int is_terminal;
} Trie;

typedef struct {
    Trie towels;

    char *patterns[512];
    int pattern_count;
} Input;

Input load_input() {
    FILE *fh = fopen("input_files/day19.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    fseek(fh, 0L, SEEK_END);
    size_t size = (size_t)ftell(fh);
    rewind(fh);

    char *buf = malloc(size + 1);

    size_t read_bytes = fread(buf, 1, size + 1, fh);

    assert(read_bytes == size);

    fclose(fh);

    buf[size] = '\0';

    Input result = { 0 };

    Trie *ptr = &result.towels;

    // Read towels
    int offset = 0;
    for (;buf[offset] != '\n'; offset++) {
        if (buf[offset] == ',') {
            ptr = &result.towels;
            offset++;
            continue;
        }

        if (!ptr->links[(int)buf[offset]]) {
            ptr->links[(int)buf[offset]] = calloc(sizeof(struct Trie), 1);
        }

        if (buf[offset + 1] == ',' || buf[offset + 1] == '\n') {
            ptr->links[(int)buf[offset]]->is_terminal = 1;
        }

        ptr = ptr->links[(int)buf[offset]];
    }

    // Skip empty line
    while (buf[offset] == '\n') {
        offset++;
    }

    do {
        result.patterns[result.pattern_count++] = &buf[offset];
        while (buf[offset] != '\n') {
            offset++;
        }
        assert(buf[offset] == '\n');
        buf[offset] = '\0';
        offset++;
    } while (buf[offset] != '\0');

    return result;
}

int solvable(char *string, Trie *dict, int offset) {
    Trie *ptr = dict;
    while (string[offset] != '\0' && ptr->links[(int)string[offset]]) {
        if (ptr->links[(int)string[offset]]->is_terminal) {
            if (solvable(string, dict, offset + 1)) {
                return 1;
            }
        }

        ptr = ptr->links[(int)string[offset]];
        offset++;
    }

    if (string[offset] == '\0') {
        return 1;
    } else {
        return 0;
    }
}

void part1() {
    Input input = load_input();

    int result = 0;

    for (int i = 0; i < input.pattern_count; i++) {
        if (solvable(input.patterns[i], &input.towels, 0)) {
            result++;
        }
    }

    printf("Part 1 solvable count: %d\n", result);
}

uint64_t solvable_pt2(char *string, Trie *dict, int offset, uint64_t *cache) {
    int initial_offset = offset;

    if (string[offset] == '\0') {
        return 0;
    }

    if (cache[offset] != UINT64_MAX) {
        return cache[offset];
    }

    Trie *ptr = dict;

    uint64_t result = 0;

    while (string[offset] != '\0' && ptr->links[(int)string[offset]]) {
        if (ptr->links[(int)string[offset]]->is_terminal) {
            uint64_t old_result = result;
            uint64_t to_add = solvable_pt2(string, dict, offset + 1, cache);
            result += to_add;
            assert(old_result <= result);
        }

        ptr = ptr->links[(int)string[offset]];
        offset++;
    }

    if (string[offset] == '\0') {
        if (ptr->is_terminal) {
            result += 1;
        }
    }

    assert(result != UINT64_MAX);
    assert(offset < 4096);
    cache[initial_offset] = result;

    return result;
}

void part2() {
    Input input = load_input();

    uint64_t result = 0;

    for (int i = 0; i < input.pattern_count; i++) {
        uint64_t cache[4096];
        for (int j = 0; j < alen(cache); j++) {
            cache[j] = UINT64_MAX;
        }


        uint64_t next_result = solvable_pt2(input.patterns[i], &input.towels, 0, cache);
        uint64_t old_result = result;
        result += next_result;

        assert(old_result <= result);
    }

    printf("Part 2 total solvable count: %lu", result);
    printf("\n");
}


int main() {
    part1();
    part2();

    return 0;
}
