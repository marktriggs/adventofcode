// I didn't end up using this code!  Eventually I realised I wasn't going to
// find the answer automatically for this input, so I ended up using day24.rb to
// print the expanded values for each output pin, and then worked it through by
// hand to find the swapped gates (using the fact that it's a ripple carry adder
// with a predictable structure, plus a bit of trial-and-error).
//


// Current idea: sample lots of random additions.  Find the output bits that are
// right, backpropagate through all gates and increase a "correct" counter.
// Find the output bits that are wrong, and bump "incorrect" counter.  Do some
// gates end up wrong more often?

#include <sys/types.h>
#include <unistd.h>

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <stdint.h>

#include "lib.c"

typedef struct {
    char label[4];
    int value;
} wire;


uint64_t hash_wire(void *wire) {
    char *wire_label = wire;
    const uint64_t prime = 16777619;

    uint64_t hash = 2166136261;

    for (int i = 0; i < 4; i++) {
        hash = (hash ^ (uint64_t)wire_label[i]) * prime;
    }

    return hash;
}

int cmp_wire(void *a, void *b) {
    return strcmp((char *) a, (char*) b);
}


int set_wire(hash_table *table, char *label, int value) {
    char *key = calloc(1, strlen(label) + 1);
    strcpy(key, label);

    wire *wire_value = calloc(1, sizeof(wire));
    strcpy(wire_value->label, label);
    wire_value->value = value;

    wire *old_value = (wire *)set_hash(table, (void *)key, (void *)wire_value);

    if (old_value != NULL) {
        assert(old_value->value == value);
        free(old_value);
        return 0;
    } else {
        return 1;
    }
}

wire *find_wire(hash_table *table, char *label) {
    return (wire *)get_hash(table, label);
}


void part1() {
    FILE *fh = fopen("input_files/day24.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    hash_table wire_set = (hash_table) {
        .slots = calloc(1, sizeof(hash_entry *) * 26),
        .slot_count = 26,
        .hash_key = hash_wire,
        .key_cmp = cmp_wire,
        .free_key = free,
        .free_value = free,
    };


    char buf[4096];

    int running = 1;
    int wires_initialised = 0;

    while (running) {
        fseek(fh, 0L, SEEK_SET);

        for (;;) {
            if (fgets(buf, sizeof buf - 1, fh) == NULL) {
                break;
            }

            if (strcmp(buf, "\n") == 0) {
                wires_initialised = 1;
                running = 0;
                continue;
            }

            char label[4];
            int value;

            if (sscanf(buf, "%3s: %d", label, &value) == 2) {
                if (!wires_initialised) {
                    set_wire(&wire_set, label, value);
                }
            } else {
                char op1[4], op2[4], out[4];

                if (sscanf(buf, "%3s AND %3s -> %3s", op1, op2, out) == 3) {
                    wire *op1_wire = find_wire(&wire_set, op1);
                    wire *op2_wire = find_wire(&wire_set, op2);

                    if (op1_wire && op2_wire) {
                        running |= set_wire(&wire_set, out, op1_wire->value & op2_wire->value);
                    }
                } else if (sscanf(buf, "%3s OR %3s -> %3s", op1, op2, out) == 3) {
                    wire *op1_wire = find_wire(&wire_set, op1);
                    wire *op2_wire = find_wire(&wire_set, op2);

                    if (op1_wire && op2_wire) {
                        running |= set_wire(&wire_set, out, op1_wire->value | op2_wire->value);
                    }
                } else if (sscanf(buf, "%3s XOR %3s -> %3s", op1, op2, out) == 3) {
                    wire *op1_wire = find_wire(&wire_set, op1);
                    wire *op2_wire = find_wire(&wire_set, op2);

                    if (op1_wire && op2_wire) {
                        running |= set_wire(&wire_set, out, op1_wire->value ^ op2_wire->value);
                    }
                } else {
                    fprintf(stderr, "Unmatched line: %s\n", buf);
                    assert(0);
                }
            }
        }
    }

    fclose(fh);

    for (uint64_t i = 0; i < wire_set.slot_count; i++) {
        hash_entry *entry = wire_set.slots[i];

        while (entry) {
            if (((char *)entry->key)[0] == 'z') {
                printf("%s: %d\n", (char *)entry->key, ((wire *)entry->value)->value);
            }

            entry = entry->next;
        }
    }
}

typedef struct {
    char in1[4];
    char in2[4];
    char operation[4];
    char output[4];

    int correct_count;
    int incorrect_count;

    int complete;
} gate;


uint64_t gate_hash(gate *gate) {
    const uint64_t prime = 16777619;

    uint64_t hash = 2166136261;

    for (int i = 0; i < 4; i++) {
        hash = (hash ^ (uint64_t)gate->in1[i]) * prime;
        hash = (hash ^ (uint64_t)gate->in2[i]) * prime;
        hash = (hash ^ (uint64_t)gate->operation[i]) * prime;
        hash = (hash ^ (uint64_t)gate->output[i]) * prime;
    }

    return hash;
}

void propagate_result(hash_table *gates_by_output, char *label, int was_correct) {
    gate *g = get_hash(gates_by_output, label);

    if (g) {
        if (was_correct) {
            g->correct_count++;
        } else {
            g->incorrect_count++;
        }

        propagate_result(gates_by_output, g->in1, was_correct);
        propagate_result(gates_by_output, g->in2, was_correct);
    }
}

void bless_gate(hash_table *gates_by_output, char *label) {
    gate *g = get_hash(gates_by_output, label);

    if (g) {
        g->incorrect_count = 0;

        bless_gate(gates_by_output, g->in1);
        bless_gate(gates_by_output, g->in2);
    }
}


typedef struct {
    char expected[4096];
    char actual[4096];
} log_entry;


void part2() {
    FILE *fh = fopen("input_files/day24.txt", "r");

    int input_bit_width = 45;
    int output_bit_width = 46;

    if (fh == NULL) {
        perror("File not found");
    }

    hash_table gates_by_output = (hash_table) {
        .slots = calloc(4096, sizeof(hash_entry *)),
        .slot_count = 4096,
        .hash_key = hash_wire,
        .key_cmp = cmp_wire,
        .free_key = NULL,
        .free_value = free,
    };

    char buf[4096];

    for (;;) {
        if (fgets(buf, sizeof buf - 1, fh) == NULL) {
            break;
        }

        if (strcmp(buf, "\n") == 0) {
            continue;
        }

        char label[4];
        int value;

        if (sscanf(buf, "%3s: %d", label, &value) == 2) {
            // We don't care about these values anymore
        } else {
            char op1[4], op2[4], out[4];

            gate *g = calloc(1, sizeof(gate));

            if (sscanf(buf, "%3s AND %3s -> %3s", op1, op2, out) == 3) {
                strcpy(g->in1, op1);
                strcpy(g->in2, op2);
                strcpy(g->operation, "AND");
                strcpy(g->output, out);
            } else if (sscanf(buf, "%3s OR %3s -> %3s", op1, op2, out) == 3) {
                strcpy(g->in1, op1);
                strcpy(g->in2, op2);
                strcpy(g->operation, "OR");
                strcpy(g->output, out);
            } else if (sscanf(buf, "%3s XOR %3s -> %3s", op1, op2, out) == 3) {
                strcpy(g->in1, op1);
                strcpy(g->in2, op2);
                strcpy(g->operation, "XOR");
                strcpy(g->output, out);
            } else {
                fprintf(stderr, "Unmatched line: %s\n", buf);
                assert(0);
            }

            assert(!set_hash(&gates_by_output, g->output, g));
        }
    }

    fclose(fh);

    hash_table wire_set = (hash_table) {
        .slots = calloc(4096, sizeof(hash_entry *)),
        .slot_count = 4096,
        .hash_key = hash_wire,
        .key_cmp = cmp_wire,
        .free_key = free,
        .free_value = free,
    };

    srand((unsigned int)getpid());

    int max_log_entries = 4096;
    log_entry *output_log = calloc((unsigned long)output_bit_width, sizeof(log_entry));

    int max_iterations = 5000;

    for (int iteration = 0; iteration < max_iterations; iteration++) {
        hash_clear(&wire_set);

        uint64_t input_a = ((uint64_t)rand() << 32) | (uint64_t) rand();
        uint64_t input_b = ((uint64_t)rand() << 32) | (uint64_t) rand();

        /* printf("A: %lu; B: %lu\n", input_a, input_b); */

        for (int i = 0; i < input_bit_width; i++) {
            char label[4];

            sprintf(label, "x%02d", i);
            set_wire(&wire_set, label, (input_a >> i) & 0x01);

            sprintf(label, "y%02d", i);
            set_wire(&wire_set, label, (input_b >> i) & 0x01);
        }

        // Reset the gates
        for (uint64_t i = 0; i < gates_by_output.slot_count; i++) {
            hash_entry *entry = gates_by_output.slots[i];

            while (entry) {
                gate *g = ((gate *)entry->value);
                g->complete = 0;

                entry = entry->next;
            }
        }

        for (;;) {
            int progressed = 0;

            for (uint64_t i = 0; i < gates_by_output.slot_count; i++) {
                hash_entry *entry = gates_by_output.slots[i];

                while (entry) {
                    gate *g = ((gate *)entry->value);

                    if (!g->complete) {
                        wire *wire_a = find_wire(&wire_set, g->in1);
                        wire *wire_b = find_wire(&wire_set, g->in2);

                        if (wire_a && wire_b) {
                            int value = -1;

                            if (g->operation[0] == 'A') {
                                value = wire_a->value & wire_b->value;
                            } else if (g->operation[0] == 'O') {
                                value = wire_a->value | wire_b->value;
                            } else if (g->operation[0] == 'X') {
                                value = wire_a->value ^ wire_b->value;
                            }

                            assert(value >= 0);

                            set_wire(&wire_set, g->output, value);
                            g->complete = 1;

                            progressed = 1;
                        }
                    }

                    entry = entry->next;
                }
            }

            if (!progressed) {
                break;
            }
        }

        // Find our output errors and backpropagate counts through the gates
        uint64_t correct_result = input_a + input_b;

        for (int i = 0; i < output_bit_width; i++) {
            char label[4];

            sprintf(label, "z%02d", i);

            int computed_value = find_wire(&wire_set, label)->value;
            int expected_value = ((correct_result >> i) & 0x01);

            if (iteration < max_log_entries) {
                output_log[i].expected[iteration] = (char)expected_value;
                output_log[i].actual[iteration] = (char)computed_value;
            }

            int was_correct = (computed_value == expected_value);

            propagate_result(&gates_by_output, label, was_correct);
        }
    }

    // If a gate was right every time, then all of the gates that came before it
    // must have been right too
    for (uint64_t i = 0; i < gates_by_output.slot_count; i++) {
        hash_entry *entry = gates_by_output.slots[i];

        while (entry) {
            gate *g = ((gate *)entry->value);

            if (g->incorrect_count == 0) {
                bless_gate(&gates_by_output, g->output);
            }

            entry = entry->next;
        }
    }

    for (uint64_t i = 0; i < gates_by_output.slot_count; i++) {
        hash_entry *entry = gates_by_output.slots[i];

        while (entry) {
            gate *g = ((gate *)entry->value);

            if (g->incorrect_count > 0) {
                double badness_score = (double)g->incorrect_count / (double)(g->correct_count + g->incorrect_count);

                printf("%.5f Gate %s %s %s -> %s correct=%d incorrect=%d\n", badness_score, g->in1, g->operation, g->in2, g->output, g->correct_count, g->incorrect_count);
            }

            entry = entry->next;
        }
    }

    hash_clear(&wire_set);
    free(wire_set.slots);

    hash_clear(&gates_by_output);
    free(gates_by_output.slots);

    int entry_count = max_log_entries;
    if (max_iterations < entry_count) {
        entry_count = max_iterations;
    }

    for (int out = 0; out < output_bit_width; out++) {
        printf("EXPECTED[%d]: ", out);
        for (int i = 0; i < entry_count; i++) {
            if (output_log[out].expected[i] == 0) {
                printf("0");
            } else {
                printf("1");
            }
        }
        printf("\n");
        printf("ACTUAL[%d]: ", out);
        for (int i = 0; i < entry_count; i++) {
            if (output_log[out].actual[i] == 0) {
                printf("0");
            } else {
                printf("1");
            }
        }
        printf("\n");
    }
}


int main() {
    // part1();
    part2();
}
