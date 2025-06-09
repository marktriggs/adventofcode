#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <assert.h>
#include <stdint.h>

#include "lib.c"


typedef struct Stone {
    uint64_t value;
    struct Stone *next;
    struct Stone *prev;
} Stone;

typedef struct {
    size_t max;
    size_t len;
    char *bytes;
} Arena;

Arena arena_make(size_t max) {
    return (Arena) {
        .max = max,
        .len = 0,
        .bytes = malloc(max),
    };
}

void *arena_alloc(Arena *a, size_t size) {
    assert((a->len + size) <= a->max);

    char *result = a->bytes + a->len;
    a->len += size;

    return result;
}

void arena_free(Arena a) {
    free(a.bytes);
}

char *load_input(Arena *arena) {
    FILE *fh = fopen("input_files/day11.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    fseek(fh, 0L, SEEK_END);
    size_t size = (size_t)ftell(fh);
    rewind(fh);

    char *result = arena_alloc(arena, size + 1);

    size_t read_bytes = fread(result, 1, size, fh);
    assert(read_bytes == size);

    fclose(fh);

    return result;
}


Stone *parse_stones(Arena *a, char *input) {
    int offset = 0;

    Stone *last_stone = NULL;

    while (input[offset] != '\0' && input[offset] != '\n') {
        uint64_t value = readint(input, &offset);

        Stone *s = arena_alloc(a, sizeof(Stone));

        s->value = value;
        s->prev = last_stone;
        s->next = NULL;

        if (last_stone) {
            last_stone->next = s;
        }

        last_stone = s;

        if (input[offset] == ' ') {
            offset++;
        }
    }

    while (last_stone->prev) {
        last_stone = last_stone->prev;
    }

    return last_stone;
}

void show_stones(Stone *stone) {
    while (stone) {
        printf("%ld\n", stone->value);
        stone = stone->next;
    }
}

uint64_t solve_pt1(char *input, int blinks) {
    // shrug
    Arena arena = arena_make(1024 * 1024 * 1024);

    Stone *first_stone = parse_stones(&arena, input);

    for (int blink = 0; blink < blinks; blink++) {
        Stone *stone = first_stone;
        while (stone != NULL) {
            if (stone->value == 0) {
                stone->value = 1;
                stone = stone->next;
            } else if (intlog10(stone->value) % 2 == 0) {
                // split
                int split_length = intlog10(stone->value) / 2;

                uint64_t lhs = stone->value;
                for (int i = 0; i < split_length; i++) {
                    lhs /= 10;
                }

                uint64_t rhs = 0;
                {
                    uint64_t n = stone->value;
                    for (int i = 0, exp = 1; i < split_length; i++, exp *= 10) {
                        rhs += (n % 10) * (uint64_t)exp;
                        n /= 10;
                    }
                }

                Stone *next = stone->next;

                stone->value = lhs;

                Stone *new_stone = arena_alloc(&arena, sizeof(Stone));
                new_stone->value = rhs;

                new_stone->next = next;
                new_stone->prev = stone;
                if (next) {
                    next->prev = new_stone;
                }

                stone->next = new_stone;

                stone = next;
            } else {
                stone->value *= 2024;
                stone = stone->next;
            }
        }
    }

    // show_stones(first_stone);

    uint64_t result = 0;
    for (Stone *stone = first_stone; stone; stone = stone->next) {
        result++;
    }

    arena_free(arena);

    return result;
}

void part1() {
    Arena arena = arena_make(128 * 1024 * 1024);
    printf("Part 1: %ld stones in total\n", solve_pt1(load_input(&arena), 25));
    arena_free(arena);
}



typedef struct {
    uint64_t n;
    int blinks_remaining;
} Calculation;

typedef struct {
    int occupied;
    Calculation key;
    uint64_t value;
} CacheEntry;

typedef struct {
    CacheEntry entries[1 << 20];
} Cache;

typedef struct {
    int hit;
    uint64_t value;
} CacheHit;

int calculation_equal(Calculation *a, Calculation *b) {
    return (a->blinks_remaining == b->blinks_remaining && a->n == b->n);
}

size_t calculation_hash(Calculation *key) {
    {
        unsigned long hash = 5381;

        for (int b = 0; b < 8; b++) {
            hash = ((hash << 5) + hash) + (key->n >> (b * 8) & 0xFF);
        }

        for (int b = 0; b < 4; b++) {
            hash = ((hash << 5) + hash) + (key->blinks_remaining >> (b * 8) & 0xFF);
        }

        return hash;
    }
}

CacheHit cache_lookup(Cache *cache, Calculation *key) {
    size_t idx = calculation_hash(key) & (alen(cache->entries) - 1);
    size_t start_idx = idx;

    for (;;) {
        CacheEntry *entry = &cache->entries[idx];

        if (entry->occupied) {
            if (calculation_equal(&entry->key, key)) {
                return (CacheHit) {
                    .hit = 1,
                    .value = entry->value,
                };
            } else {
                idx += 1;
                idx &= (alen(cache->entries) - 1);

                if (idx == start_idx) {
                    break;
                }
            }
        }

        break;
    }

    return (CacheHit) {
        .hit = 0,
        .value = 0,
    };
}

int cache_insert(Cache *cache, Calculation *key, uint64_t value) {
    size_t idx = calculation_hash(key) & (alen(cache->entries) - 1);
    size_t start_idx = idx;

    for (;;) {
        CacheEntry *entry = &cache->entries[idx];

        if (entry->occupied && !calculation_equal(&entry->key, key)) {
            idx += 1;
            idx &= (alen(cache->entries) - 1);

            if (idx == start_idx) {
                break;
            }
        } else {
            entry->occupied = 1;
            entry->key = *key;
            entry->value = value;

            return 1;
        }
    }

    // Cache full!
    return 0;
}


uint64_t solve_pt2(Cache *cache, uint64_t n, int blinks_left, uint64_t stones_generated) {
    if (blinks_left == 0) {
        return stones_generated;
    }

    Calculation cache_key = (Calculation) { .blinks_remaining = blinks_left, .n = n };

    CacheHit hit = cache_lookup(cache, &cache_key);
    if (hit.hit) {
        return hit.value;
    }

    uint64_t result;

    if (n == 0) {
        result = solve_pt2(cache, 1, blinks_left - 1, stones_generated);
    } else if (intlog10(n) % 2 == 0) {
        // split
        int split_length = intlog10(n) / 2;

        uint64_t lhs = n;
        for (int i = 0; i < split_length; i++) {
            lhs /= 10;
        }

        uint64_t rhs = 0;
        {
            uint64_t remaining = n;
            for (int i = 0, exp = 1; i < split_length; i++, exp *= 10) {
                rhs += (remaining % 10) * (uint64_t)exp;
                remaining /= 10;
            }
        }

        result = solve_pt2(cache, lhs, blinks_left - 1, stones_generated) + solve_pt2(cache, rhs, blinks_left - 1, stones_generated) + 1;
    } else {
        result = solve_pt2(cache, n * 2024, blinks_left - 1, stones_generated);
    }

    assert(cache_insert(cache, &cache_key, result));

    return result;
}

void part2() {
    Arena arena = arena_make(128 * 1024 * 1024);
    Cache *cache = arena_alloc(&arena, sizeof(Cache));
    bzero(cache, sizeof(Cache));

    char *input = load_input(&arena);
    int offset = 0;

    uint64_t stones = 0;

    while (input[offset] != '\n' && input[offset] != '\0') {
        stones++;
        uint64_t stone_value = readint(input, &offset);
        stones += solve_pt2(cache, stone_value, 75, 0);
        offset++;
    }

    arena_free(arena);

    printf("Part 2: %ld stones in total\n", stones);
}


int main() {
    part1();
    part2();

    return 0;
}
