#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <stdint.h>

#include "lib.c"

void part1_wrong() {
    FILE *fh = fopen("input_files/day23_sample.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    char names[4096] = { 0 };
    int names_count = 0;

    int connections[65535];
    int set_id_count = 0;

    for (int i = 0; i < alen(connections); i++) {
        connections[i] = -1;
    }

    char buf[16];

    for (;;) {
        if (fgets(buf, sizeof buf - 1, fh) == NULL) {
            break;
        }

        if (strcmp(buf, "\n") == 0) {
            break;
        }

        int node_a = (buf[0] << 8) | buf[1];
        int node_b = (buf[3] << 8) | buf[4];

        if (connections[node_a] < 0) {
            names[names_count++] = buf[0];
            names[names_count++] = buf[1];
            connections[node_a] = set_id_count++;
        }

        if (connections[node_b] < 0) {
            names[names_count++] = buf[3];
            names[names_count++] = buf[4];
            connections[node_b] = set_id_count++;
        }

        int merge_target_set_id = connections[node_b];
        connections[node_b] = connections[node_a];

        for (int i = 0; i < alen(connections); i++) {
            if (connections[i] == merge_target_set_id) {
                connections[i] = connections[node_a];
            }
        }
    }

    fclose(fh);

    // How many sets of three are there?
    int set_sizes[1024] = { 0 };

    for (int i = 0; i < alen(connections); i++) {
        if (connections[i] >= 0) {
            set_sizes[connections[i]]++;
        }
    }

    for (int i = 0; i < alen(set_sizes); i++) {
        if (set_sizes[i] == 3) {
            printf("That's a set of three\n");
        } else {
            printf("Set size was: %d\n", set_sizes[i]);
        }
    }
}

// 10 bits
int node_hash(char *id) {
    int result = ((id[0] - 'a') << 5) | (id[1] - 'a');

    assert(result < (1 << 10));

    return result;
}

typedef struct {
    char label[3];
} label;

label node_label(int node_id) {
    label result = { 0 };

    result.label[0] = (char)((node_id >> 5) + 'a');
    result.label[1] = (char)((node_id & (1 << 5) - 1) + 'a');

    return result;
}


// 20 bits
int connection_hash(int node_a, int node_b) {
    if (node_a <= node_b) {
        return (node_a << 10) | node_b;
    } else {
        return connection_hash(node_b, node_a);
    }
}

typedef struct neighbour_list {
    int node_id;
    struct neighbour_list *next;
} neighbour_list;


int hash_group(int a, int b, int c) {
    if (a > b) { return hash_group(b, a, c); }
    if (b > c) { return hash_group(a, c, b); }

    assert(a <= b && b <= c);

    return (a << 20) | (b << 10) | c;
}


void part1() {
    FILE *fh = fopen("input_files/day23.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    int unique_nodes[1024] = { 0 };

    neighbour_list * neighbours[1024] = { 0 };

    int connections[1024 * 1024] = { 0 };

    char buf[16];

    for (;;) {
        if (fgets(buf, sizeof buf - 1, fh) == NULL) {
            break;
        }

        if (strcmp(buf, "\n") == 0) {
            break;
        }

        int node_a = node_hash(buf);
        int node_b = node_hash(buf + 3);

        connections[connection_hash(node_a, node_b)] = 1;

        neighbour_list *entry_a = malloc(sizeof(neighbour_list));
        entry_a->node_id = node_b;
        entry_a->next = neighbours[node_a];
        neighbours[node_a] = entry_a;

        neighbour_list *entry_b = malloc(sizeof(neighbour_list));
        entry_b->node_id = node_a;
        entry_b->next = neighbours[node_b];
        neighbours[node_b] = entry_b;

        assert(node_a < 1024);
        assert(node_b < 1024);

        unique_nodes[node_a] = 1;
        unique_nodes[node_b] = 1;
    }

    fclose(fh);

    char *sets_of_three = calloc(1 << 30, 1);
    int interesting_set_count = 0;

    for (int i = 0; i < alen(unique_nodes); i++) {
        if (unique_nodes[i] == 0) {
            continue;
        }

        if (node_label(i).label[0] != 't') {
            // Only care about connections starting with 't'
            continue;
        }

        neighbour_list *neighbours_a = neighbours[i];

        while (neighbours_a) {
            neighbour_list *neighbours_b = neighbours_a->next;

            while (neighbours_b) {
                if (connections[connection_hash(neighbours_a->node_id, neighbours_b->node_id)]) {
                    int group_hash = hash_group(i, neighbours_a->node_id, neighbours_b->node_id);

                    assert(group_hash < (1 << 30));

                    if (sets_of_three[group_hash]) {
                        // seen this three already
                    } else {
                        printf("%s -- %s -- %s\n",
                               node_label(i).label,
                               node_label(neighbours_a->node_id).label,
                               node_label(neighbours_b->node_id).label);

                        sets_of_three[group_hash] = 1;
                        interesting_set_count += 1;
                    }
                }

                neighbours_b = neighbours_b->next;
            }

            neighbours_a = neighbours_a->next;
        }
    }

    printf("Number of interesting sets: %d\n", interesting_set_count);
}

int find_biggest_clique(int *connections, neighbour_list *neighbours, neighbour_list *current_clique) {
    if (!neighbours) {
        neighbour_list *c = current_clique;
        int result = 0;

        while (c) {
            result++;
            c = c->next;
        }

        // And then print...
        printf("%d: ", result);

        c = current_clique;

        while (c) {
            printf("%s,", node_label(c->node_id).label);
            c = c->next;
        }

        printf("\n");

        return result;
    }

    // If the current neighbour connects to the current clique, consider it.
    neighbour_list *c = current_clique;
    int matched = 1;

    while (c) {
        if (!connections[connection_hash(neighbours->node_id, c->node_id)]) {
            matched = 0;
            break;
        }

        c = c->next;
    }

    // Excluding the current neighbour
    int best = find_biggest_clique(connections, neighbours->next, current_clique);

    if (matched) {
        neighbour_list next_clique;
        next_clique.node_id = neighbours->node_id;
        next_clique.next = current_clique;

        int candidate = find_biggest_clique(connections, neighbours->next, &next_clique);

        if (candidate > best) {
            best = candidate;
        }
    }

    return best;
}

void part2() {
    FILE *fh = fopen("input_files/day23.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    int unique_nodes[1024] = { 0 };

    neighbour_list * neighbours[1024] = { 0 };

    int connections[1024 * 1024] = { 0 };

    char buf[16];

    for (;;) {
        if (fgets(buf, sizeof buf - 1, fh) == NULL) {
            break;
        }

        if (strcmp(buf, "\n") == 0) {
            break;
        }

        int node_a = node_hash(buf);
        int node_b = node_hash(buf + 3);

        connections[connection_hash(node_a, node_b)] = 1;

        neighbour_list *entry_a = malloc(sizeof(neighbour_list));
        entry_a->node_id = node_b;
        entry_a->next = neighbours[node_a];
        neighbours[node_a] = entry_a;

        neighbour_list *entry_b = malloc(sizeof(neighbour_list));
        entry_b->node_id = node_a;
        entry_b->next = neighbours[node_b];
        neighbours[node_b] = entry_b;

        assert(node_a < 1024);
        assert(node_b < 1024);

        unique_nodes[node_a] = 1;
        unique_nodes[node_b] = 1;
    }

    fclose(fh);

    int best = 0;

    for (int i = 0; i < alen(unique_nodes); i++) {
        if (unique_nodes[i] == 0) {
            continue;
        }

        int biggest = find_biggest_clique(connections, neighbours[i], &(neighbour_list){ .node_id = i, .next = NULL });

        if (biggest > best) {
            best = biggest;
        }
    }

    printf("Best clique was: %d\n", best);
}


int main() {
    // part1();
    part2();
}
