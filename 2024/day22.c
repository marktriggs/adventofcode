#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <stdint.h>

#include "lib.c"

uint64_t iterate_secret(uint64_t seed, int iterations) {
    uint64_t secret = seed;

    uint64_t prune_mask = ((1 << 24) - 1);

    for (int i = 0; i < iterations; i++) {
        secret = (secret ^ (secret << 6)) & prune_mask;
        secret = (secret ^ (secret >> 5)) & prune_mask;
        secret = (secret ^ (secret << 11)) & prune_mask;
    }

    return secret;
}

void part1() {
    FILE *fh = fopen("input_files/day22.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    uint64_t total = 0;

    char buf[4096];

    for (;;) {
        if (fgets(buf, sizeof buf - 1, fh) == NULL) {
            break;
        }

        if (strcmp(buf, "\n") == 0) {
            break;
        }

        int readpos = 0;
        uint64_t seed = readint(buf, &readpos);

        uint64_t secret = iterate_secret(seed, 2000);

        total += secret;
    }

    fclose(fh);

    printf("Total: %ld\n", total);
}

typedef struct {
    int8_t diffs[4];
} Sequence;

typedef struct {
    Sequence sequence;
    uint8_t price;
    uint8_t occupied;
} SequencePrice;

typedef struct {
    SequencePrice table[65536];
} Buyer;


// fnv hash
uint64_t sequence_hash(Sequence sequence) {
    const uint64_t prime = 16777619;

    uint64_t hash = 2166136261;

    hash = (hash ^ (uint64_t)sequence.diffs[0]) * prime;
    hash = (hash ^ (uint64_t)sequence.diffs[1]) * prime;
    hash = (hash ^ (uint64_t)sequence.diffs[2]) * prime;
    hash = (hash ^ (uint64_t)sequence.diffs[3]) * prime;

    return hash;
}


int sequence_equals(Sequence a, Sequence b) {
    for (int i = 0; i < alen(a.diffs); i++) {
        if (a.diffs[i] != b.diffs[i]) {
            return 0;
        }
    }

    return 1;
}

SequencePrice buyer_sequence_lookup(Buyer *buyer, Sequence sequence) {
    uint64_t hash = sequence_hash(sequence);

    int pos = hash % alen(buyer->table);

    for (int i = 0; i < alen(buyer->table); i++) {
        if (buyer->table[pos].occupied && sequence_equals(buyer->table[pos].sequence, sequence)) {
            return buyer->table[pos];
        } else if (!buyer->table[pos].occupied) {
            return buyer->table[pos];
        } else {
            pos = (pos + 1) % alen(buyer->table);
        }
    }

    // Hash table full
    assert(0);
}

void store_price_for_sequence(Buyer *buyer, Sequence sequence, uint8_t price) {
    uint64_t hash = sequence_hash(sequence);
    int pos = hash % alen(buyer->table);

    for (int i = 0; i < alen(buyer->table); i++) {
        if (!buyer->table[pos].occupied) {
            buyer->table[pos].occupied = 1;
            buyer->table[pos].price = price;
            buyer->table[pos].sequence = sequence;

            return;
        } else {
            pos = (pos + 1) % alen(buyer->table);
        }
    }

    // Hash table full
    assert(0);
}

uint64_t load_buyer_sequence(Buyer *buyer, uint64_t seed, int iterations) {
    uint64_t secret = seed;

    uint64_t prune_mask = ((1 << 24) - 1);
    uint8_t last_price = 0;

    Sequence sequence = { 0 };

    for (int i = 0; i < iterations; i++) {
        secret = (secret ^ (secret << 6)) & prune_mask;
        secret = (secret ^ (secret >> 5)) & prune_mask;
        secret = (secret ^ (secret << 11)) & prune_mask;

        uint8_t price = secret % 10;

        if (i > 0) {
            int8_t diff = ((int8_t)price - (int8_t)last_price);

            if (i <= 4) {
                sequence.diffs[i - 1] = diff;
            } else {
                sequence.diffs[0] = sequence.diffs[1];
                sequence.diffs[1] = sequence.diffs[2];
                sequence.diffs[2] = sequence.diffs[3];
                sequence.diffs[3] = diff;

                // Store the new sequence for the current price if we haven't seen it yet.
                if (!buyer_sequence_lookup(buyer, sequence).occupied) {
                    store_price_for_sequence(buyer, sequence, price);
                }
            }
        }

        last_price = price;
    }

    return secret;
}

int count_bananas(Buyer *buyers, int buyer_count, Sequence candidate_sequence) {
    int result = 0;

    for (int i = 0; i < buyer_count; i++) {
        SequencePrice price = buyer_sequence_lookup(&buyers[i], candidate_sequence);

        if (price.occupied) {
            result += price.price;
        }
    }

    return result;
}

void part2() {
    FILE *fh = fopen("input_files/day22.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    Buyer *buyers = calloc(2048, sizeof(Buyer));

    char buf[4096];

    int buyer_count = 0;
    for (;;) {
        if (fgets(buf, sizeof buf - 1, fh) == NULL) {
            break;
        }

        if (strcmp(buf, "\n") == 0) {
            break;
        }

        int readpos = 0;
        uint64_t seed = readint(buf, &readpos);

        load_buyer_sequence(&buyers[buyer_count], seed, 2000);

        buyer_count++;
    }

    // for (int i = 0; i < buyer_count; i++) {
    //     for (int table_idx = 0; table_idx < alen(buyers[i].table); table_idx++) {
    //         if (buyers[i].table[table_idx].occupied) {
    //             printf("%d %d %d %d\n",
    //                    buyers[i].table[table_idx].sequence.diffs[0],
    //                    buyers[i].table[table_idx].sequence.diffs[1],
    //                    buyers[i].table[table_idx].sequence.diffs[2],
    //                    buyers[i].table[table_idx].sequence.diffs[3]);
    //         }
    //     }
    // }


    int best_bananas = INT_MIN;

    Buyer merged_buyers = { 0 };

    for (int i = 0; i < buyer_count; i++) {
        for (int table_idx = 0; table_idx < alen(buyers[i].table); table_idx++) {
            if (buyers[i].table[table_idx].occupied && !buyer_sequence_lookup(&merged_buyers, buyers[i].table[table_idx].sequence).occupied) {
                int bananas = count_bananas(buyers, buyer_count, buyers[i].table[table_idx].sequence);

                if (bananas > best_bananas) {
                    best_bananas = bananas;
                }

                // Mark it off
                store_price_for_sequence(&merged_buyers, buyers[i].table[table_idx].sequence, 1);
            }
        }
    }

    printf("Part 2 total bananas possible: %d\n", best_bananas);

    fclose(fh);
}

int main() {
    // part1();

    part2();
}
