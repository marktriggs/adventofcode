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

#define MAX_GRID 32768

typedef struct {
    int x;
    int y;
} Point;

typedef struct {
    int items[MAX_GRID];
    int width;
    int height;
} Grid;

// NOTE: matches offsets order below
typedef enum {
    North,
    East,
    South,
    West
} Direction;

typedef struct {
    Grid grid;
    Point start;
    Point exit;
} Input;


typedef struct {
    Point position;
    int priority;
} State;

Input load_input() {
    FILE *fh = fopen("input_files/day20.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    Input result = { 0 };

    char buf[4096];

    // Read grid
    int row = 0;
    int offset = 0;

    while (fgets(buf, sizeof(buf) - 1, fh)) {
        if (buf[0] == '\n') {
            break;
        }

        int i;
        for (i = 0; buf[i] != '\n'; i++) {
            if (buf[i] == 'S') {
                result.start.x = i;
                result.start.y = row;
                result.grid.items[offset++] = '.';
            } else if (buf[i] == 'E') {
                result.exit.x = i;
                result.exit.y = row;
                result.grid.items[offset++] = '.';
            } else {
                result.grid.items[offset++] = buf[i];
            }

            assert(offset < MAX_GRID);
        }

        result.grid.width = i;
        result.grid.height++;
        row++;
    }

    fclose(fh);

    return result;
}

Point point_add(Point p1, Point p2) {
    return (Point) {
        .x = p1.x + p2.x,
        .y = p1.y + p2.y,
    };
}

int point_equals(Point p1, Point p2) {
    return (p1.x == p2.x && p1.y == p2.y);
}

int point_distance(Point p1, Point p2) {
    return abs(p1.x - p2.x) + abs(p1.y - p2.y);
}


int in_range(Input *input, Point p) {
    return (p.x >= 0 && p.y >= 0 && p.x < input->grid.width && p.y < input->grid.height);
}


int grid_position(Grid *grid, Point p) {
    return (grid->width * p.y) + p.x;
}

int grid_get(Grid *grid, Point p) {
    return grid->items[(grid->width * p.y) + p.x];
}

static Point NorthOffset = (Point) { .x = 0, .y = -1 };
static Point EastOffset = (Point) { .x = 1, .y = 0 };
static Point SouthOffset = (Point) { .x = 0, .y = 1 };
static Point WestOffset = (Point) { .x = -1, .y = 0 };

static Point offsets[4];

void solve(Input *input, Point position, Direction direction, int cost, char *seen_positions, int *best_ever) {
    if (grid_get(&input->grid, position) == 'E') {
        if (cost < *best_ever) {
            printf("Besty: %d\n", *best_ever);
            *best_ever = cost;
        }

        return;
    }

    if (cost >= *best_ever) {
        return;
    }

    if (grid_get(&input->grid, position) == 'S') {
        seen_positions[grid_position(&input->grid, position)] = 1;
    }


    for (int round = 0; round < 2; round++) {
        for (int i = 0; i < alen(offsets); i++) {
            if (round == 0 && i != (int)direction) {
                continue;
            } else if (round == 1 && i == (int)direction) {
                continue;
            }

            Point next_point = point_add(position, offsets[i]);

            if (!seen_positions[grid_position(&input->grid, next_point)] && grid_get(&input->grid, next_point) != '#') {
                int next_cost = ((int)direction == i) ? 1 : 1001;

                if (next_cost < *best_ever) {
                    seen_positions[grid_position(&input->grid, next_point)] = 1;
                    solve(input, next_point, (Direction)i, cost + next_cost, seen_positions, best_ever);
                    seen_positions[grid_position(&input->grid, next_point)] = 0;
                }

            }
        }
    }
}

#define MAX_PQ_SIZE 131072

typedef struct {
    State heap[MAX_PQ_SIZE];
    int size;
} PriorityQueue;

void pq_swap(State *a, State *b) {
    State temp = *a;
    *a = *b;
    *b = temp;
}

int pq_len(PriorityQueue *pq) {
    return pq->size;
}

void pq_push(PriorityQueue *pq, State value) {
    assert(pq->size < MAX_PQ_SIZE);

    int i = pq->size++;
    pq->heap[i] = value;

    while (i > 0 && pq->heap[(i-1)/2].priority > pq->heap[i].priority) {
        pq_swap(&pq->heap[i], &pq->heap[(i-1)/2]);
        i = (i-1)/2;
    }
}

State *pq_get(PriorityQueue *pq, State elt) {
    int i;
    for (i = 0; i < pq->size; i++) {
        if ((pq->heap[i].position.x == elt.position.x) &&
            (pq->heap[i].position.y == elt.position.y)) {

            return &pq->heap[i];
        }
    }

    assert(0);
}


int pq_get_priority(PriorityQueue *pq, State elt) {
    int i;
    for (i = 0; i < pq->size; i++) {
        if ((pq->heap[i].position.x == elt.position.x) &&
            (pq->heap[i].position.y == elt.position.y)) {

            return pq->heap[i].priority;
        }
    }

    assert(0);
}


int pq_set_priority(PriorityQueue *pq, State elt, int priority) {
    int i;
    for (i = 0; i < pq->size; i++) {
        if ((pq->heap[i].position.x == elt.position.x) &&
            (pq->heap[i].position.y == elt.position.y)) {

            break;
        }
    }

    assert(i < pq->size);

    // Pluck out and modify our entry
    State existing = pq->heap[i];
    int result = existing.priority;

    assert(priority <= existing.priority);
    existing.priority = priority;

    pq->heap[i].priority = priority;

    // Fix heap property
    // First try moving up
    while (i > 0 && pq->heap[(i-1)/2].priority > pq->heap[i].priority) {
        pq_swap(&pq->heap[i], &pq->heap[(i-1)/2]);
        i = (i-1)/2;
    }

    // Then try moving down
    while (2*i + 1 < pq->size) {
        int child = 2*i + 1;
        if (child+1 < pq->size && pq->heap[child+1].priority < pq->heap[child].priority)
            child++;
        if (pq->heap[i].priority <= pq->heap[child].priority)
            break;
        pq_swap(&pq->heap[i], &pq->heap[child]);
        i = child;
    }

    return result;
}

int pq_contains(PriorityQueue *pq, State *elt) {
    for (int i = 0; i < pq->size; i++) {
        if ((pq->heap[i].position.x == elt->position.x) &&
            (pq->heap[i].position.y == elt->position.y)) {
            return 1;
        }
    }

    return 0;
}

State pq_pop(PriorityQueue *pq) {
    State ret = pq->heap[0];
    pq->heap[0] = pq->heap[--pq->size];

    int i = 0;
    while (2*i + 1 < pq->size) {
        int child = 2*i + 1;
        if (child+1 < pq->size && pq->heap[child+1].priority < pq->heap[child].priority)
            child++;
        if (pq->heap[i].priority <= pq->heap[child].priority)
            break;
        pq_swap(&pq->heap[i], &pq->heap[child]);
        i = child;
    }
    return ret;
}


void part1() {
    // Hello again Dijkstra
    Input input = load_input();

    offsets[0] = NorthOffset;
    offsets[1] = EastOffset;
    offsets[2] = SouthOffset;
    offsets[3] = WestOffset;

    Direction opposites[4][4];
    memset(opposites, 0, sizeof(opposites));
    opposites[North][South] = 1;
    opposites[East][West] = 1;
    opposites[South][North] = 1;
    opposites[West][East] = 1;

    PriorityQueue *queue = malloc(sizeof(PriorityQueue));

    queue->size = 0;
    int dist[MAX_GRID * 4] = { 0 };

    // int best = INT_MAX;

    for (int row = 0; row < input.grid.height; row++) {
        for (int col = 0; col < input.grid.width; col++) {

            if (grid_get(&input.grid, (Point) { .x = col, .y = row }) == '#') {
                continue;
            }

            int idx = grid_position(&input.grid, (Point) { .x = col, .y = row });

            int initial_distance = INT_MAX;

            if (point_equals(input.start, (Point) { .x = col, .y = row })) {
                initial_distance = 0;
            }

            dist[idx] = initial_distance;

            State entry = (State) {
                .priority = initial_distance,
                .position = (Point) { .x = col, .y = row },
            };

            pq_push(queue, entry);
        }
    }

    while (pq_len(queue) > 0) {
        State current = pq_pop(queue);

        if (current.priority == INT_MAX) {
            break;
        }

        // if (point_equals(current.position, input.exit)) {
        //     if (current.priority <= best) {
        //         printf("Shortest: %d\n", current.priority);
        //     } else {
        //         break;
        //     }
        // }

        assert(dist[grid_position(&input.grid, current.position)] != INT_MAX);

        for (int i = 0; i < alen(offsets); i++) {
            Point next_point = point_add(current.position, offsets[i]);

            if (!in_range(&input, next_point)) {
                continue;
            }

            if (grid_get(&input.grid, next_point) == '#') {
                continue;
            }

            int next_cost = dist[grid_position(&input.grid, current.position)] + 1;
            assert(next_cost > 0);

            if (next_cost < dist[grid_position(&input.grid, next_point)]) {
                dist[grid_position(&input.grid, next_point)] = next_cost;

                State key_state = (State) { .position = next_point };
                pq_set_priority(queue, key_state, next_cost);
            }
        }
    }

    int good_savings = 0;


    for (int row = 0; row < input.grid.height; row++) {
        for (int col = 0; col < input.grid.width - 2; col++) {
            Point a = (Point) { .x = col, .y = row};
            Point b = (Point) { .x = col + 1, .y = row};
            Point c = (Point) { .x = col + 2, .y = row};

            if (grid_get(&input.grid, a) == '.' &&
                grid_get(&input.grid, b) == '#' &&
                grid_get(&input.grid, c) == '.') {
                int saving = abs(dist[grid_position(&input.grid, a)] - dist[grid_position(&input.grid, c)]) - 2;

                if (saving >= 100) {
                    good_savings++;
                }
            }
        }
    }

    for (int row = 0; row < input.grid.height; row++) {
        for (int col = 0; col < input.grid.width - 3; col++) {
            Point a = (Point) { .x = col, .y = row};
            Point b = (Point) { .x = col + 1, .y = row};
            Point c = (Point) { .x = col + 2, .y = row};
            Point d = (Point) { .x = col + 3, .y = row};

            if (grid_get(&input.grid, a) == '.' &&
                grid_get(&input.grid, b) == '#' &&
                grid_get(&input.grid, c) == '#' &&
                grid_get(&input.grid, c) == '.') {
                int saving = abs(dist[grid_position(&input.grid, a)] - dist[grid_position(&input.grid, d)]) - 3;

                if (saving >= 100) {
                    good_savings++;
                }
            }
        }
    }

    for (int row = 0; row < input.grid.height - 2; row++) {
        for (int col = 0; col < input.grid.width ; col++) {
            Point a = (Point) { .x = col, .y = row};
            Point b = (Point) { .x = col, .y = row + 1};
            Point c = (Point) { .x = col, .y = row + 2};

            if (grid_get(&input.grid, a) == '.' &&
                grid_get(&input.grid, b) == '#' &&
                grid_get(&input.grid, c) == '.') {
                int saving = abs(dist[grid_position(&input.grid, a)] - dist[grid_position(&input.grid, c)]) - 2;

                if (saving >= 100) {
                    good_savings++;
                }
            }
        }
    }

    for (int row = 0; row < input.grid.height - 3; row++) {
        for (int col = 0; col < input.grid.width; col++) {
            Point a = (Point) { .x = col, .y = row};
            Point b = (Point) { .x = col, .y = row + 1};
            Point c = (Point) { .x = col, .y = row + 2};
            Point d = (Point) { .x = col, .y = row + 3};

            if (grid_get(&input.grid, a) == '.' &&
                grid_get(&input.grid, b) == '#' &&
                grid_get(&input.grid, c) == '#' &&
                grid_get(&input.grid, d) == '.') {
                int saving = abs(dist[grid_position(&input.grid, a)] - dist[grid_position(&input.grid, d)]) - 3;

                if (saving >= 100) {
                    good_savings++;
                }
            }
        }
    }

    printf("Part 1 good savings: %d\n", good_savings);
}

void part2() {
    // Hello again Dijkstra
    Input input = load_input();

    offsets[0] = NorthOffset;
    offsets[1] = EastOffset;
    offsets[2] = SouthOffset;
    offsets[3] = WestOffset;

    Direction opposites[4][4];
    memset(opposites, 0, sizeof(opposites));
    opposites[North][South] = 1;
    opposites[East][West] = 1;
    opposites[South][North] = 1;
    opposites[West][East] = 1;

    PriorityQueue *queue = malloc(sizeof(PriorityQueue));

    queue->size = 0;
    int dist[MAX_GRID * 4] = { 0 };

    // int best = INT_MAX;

    for (int row = 0; row < input.grid.height; row++) {
        for (int col = 0; col < input.grid.width; col++) {

            if (grid_get(&input.grid, (Point) { .x = col, .y = row }) == '#') {
                continue;
            }

            int idx = grid_position(&input.grid, (Point) { .x = col, .y = row });

            int initial_distance = INT_MAX;

            if (point_equals(input.start, (Point) { .x = col, .y = row })) {
                initial_distance = 0;
            }

            dist[idx] = initial_distance;

            State entry = (State) {
                .priority = initial_distance,
                .position = (Point) { .x = col, .y = row },
            };

            pq_push(queue, entry);
        }
    }

    while (pq_len(queue) > 0) {
        State current = pq_pop(queue);

        if (current.priority == INT_MAX) {
            break;
        }

        for (int i = 0; i < alen(offsets); i++) {
            Point next_point = point_add(current.position, offsets[i]);

            if (!in_range(&input, next_point)) {
                continue;
            }

            if (grid_get(&input.grid, next_point) == '#') {
                continue;
            }

            int next_cost = dist[grid_position(&input.grid, current.position)] + 1;
            assert(next_cost > 0);

            if (next_cost < dist[grid_position(&input.grid, next_point)]) {
                dist[grid_position(&input.grid, next_point)] = next_cost;

                State key_state = (State) { .position = next_point };
                pq_set_priority(queue, key_state, next_cost);
            }
        }
    }

    int good_cheats = 0;

    for (int row = 0; row < input.grid.height; row++) {
        for (int col = 0; col < input.grid.width; col++) {
            Point start_cheat = (Point) { .x = col, .y = row};

            if (grid_get(&input.grid, start_cheat) != '.') {
                continue;
            }

            for (int row_offset = -20; row_offset <= 20; row_offset++) {
                for (int col_offset = -20; col_offset <= 20; col_offset++) {
                    if ((abs(row_offset) + abs(col_offset)) > 20) {
                        continue;
                    }

                    Point end_cheat = (Point) { .x = col + col_offset, .y = row + row_offset};

                    if (!in_range(&input, end_cheat)) {
                        continue;
                    }

                    if (grid_get(&input.grid, end_cheat) != '.') {
                        assert(grid_get(&input.grid, end_cheat) == '#');
                        continue;
                    }

                    int saving = dist[grid_position(&input.grid, end_cheat)] - dist[grid_position(&input.grid, start_cheat)] - (abs(row_offset) + abs(col_offset));

                    if (saving >= 100) {
                        good_cheats++;
                    }
                }
            }
        }
    }
    printf("Part 2 good cheats: %d\n", good_cheats);
}

int main() {
    part1();
    part2();

    return 0;
}
