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
    Point reindeer;
    Point exit;
    Direction direction;
} Input;


typedef struct Path {
    int point;
    struct Path *last;
} Path;

struct Path;

typedef struct Paths {
    struct Path *path;
    struct Paths *next;
} Paths;

typedef struct {
    Point position;
    Direction direction;
    int cost_heuristic;
    int accumulated_cost;
    char visited_points[3000];
} State;

Input load_input() {
    FILE *fh = fopen("input_files/day16.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    Input result = { 0 };
    result.direction = East;

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
                result.reindeer.x = i;
                result.reindeer.y = row;
            }

            if (buf[i] == 'E') {
                result.exit.x = i;
                result.exit.y = row;
            }

            result.grid.items[offset++] = buf[i];
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

    while (i > 0 && pq->heap[(i-1)/2].cost_heuristic > pq->heap[i].cost_heuristic) {
        pq_swap(&pq->heap[i], &pq->heap[(i-1)/2]);
        i = (i-1)/2;
    }
}

State *pq_get(PriorityQueue *pq, State elt) {
    int i;
    for (i = 0; i < pq->size; i++) {
        if ((pq->heap[i].position.x == elt.position.x) &&
            (pq->heap[i].position.y == elt.position.y) &&
            (pq->heap[i].direction == elt.direction)) {

            return &pq->heap[i];
        }
    }

    assert(0);
}


int pq_get_priority(PriorityQueue *pq, State elt) {
    int i;
    for (i = 0; i < pq->size; i++) {
        if ((pq->heap[i].position.x == elt.position.x) &&
            (pq->heap[i].position.y == elt.position.y) &&
            (pq->heap[i].direction == elt.direction)) {

            return pq->heap[i].cost_heuristic;
        }
    }

    assert(0);
}


int pq_set_priority(PriorityQueue *pq, State elt, int priority) {
    int i;
    for (i = 0; i < pq->size; i++) {
        if ((pq->heap[i].position.x == elt.position.x) &&
            (pq->heap[i].position.y == elt.position.y) &&
            (pq->heap[i].direction == elt.direction)) {

            break;
        }
    }

    assert(i < pq->size);

    // Pluck out and modify our entry
    State existing = pq->heap[i];
    int result = existing.cost_heuristic;

    assert(priority <= existing.cost_heuristic);
    existing.cost_heuristic = priority;

    pq->heap[i].cost_heuristic = priority;

    // Fix heap property
    // First try moving up
    while (i > 0 && pq->heap[(i-1)/2].cost_heuristic > pq->heap[i].cost_heuristic) {
        pq_swap(&pq->heap[i], &pq->heap[(i-1)/2]);
        i = (i-1)/2;
    }

    // Then try moving down
    while (2*i + 1 < pq->size) {
        int child = 2*i + 1;
        if (child+1 < pq->size && pq->heap[child+1].cost_heuristic < pq->heap[child].cost_heuristic)
            child++;
        if (pq->heap[i].cost_heuristic <= pq->heap[child].cost_heuristic)
            break;
        pq_swap(&pq->heap[i], &pq->heap[child]);
        i = child;
    }

    return result;
}

int pq_contains(PriorityQueue *pq, State *elt) {
    for (int i = 0; i < pq->size; i++) {
        if ((pq->heap[i].position.x == elt->position.x) &&
            (pq->heap[i].position.y == elt->position.y) &&
            (pq->heap[i].direction == elt->direction)) {
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
        if (child+1 < pq->size && pq->heap[child+1].cost_heuristic < pq->heap[child].cost_heuristic)
            child++;
        if (pq->heap[i].cost_heuristic <= pq->heap[child].cost_heuristic)
            break;
        pq_swap(&pq->heap[i], &pq->heap[child]);
        i = child;
    }
    return ret;
}


void part1() {
    // Hello A-Star
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

    State initial = (State) {
        .direction = input.direction,
        .position = input.reindeer,
        .cost_heuristic = point_distance(input.reindeer, input.exit),
        .accumulated_cost = 0,
    };

    pq_push(queue, initial);

    // For node n, gScore[n] is the currently known cost of the cheapest path from start to n.
    int gscore[MAX_GRID];
    for (int i = 0; i < alen(gscore); i++) { gscore[i] = INT_MAX; }
    gscore[grid_position(&input.grid, initial.position)] = 0;

    // For node n, fScore[n] := gScore[n] + h(n). fScore[n] represents our current best guess as to
    // how cheap a path could be from start to finish if it goes through n.
    int fscore[MAX_GRID] ;
    for (int i = 0; i < alen(fscore); i++) { fscore[i] = INT_MAX; }
    fscore[grid_position(&input.grid, initial.position)] = point_distance(input.reindeer, input.exit);

    int lowest = INT_MAX;

    while (pq_len(queue) > 0) {
        State current = pq_pop(queue);

        if (grid_get(&input.grid, current.position) == 'E') {
            lowest = current.accumulated_cost;
            break;
        }

        for (int i = 0; i < alen(offsets); i++) {
            Point next_point = point_add(current.position, offsets[i]);

            if (grid_get(&input.grid, next_point) != '#') {
                int turn_penalty = ((int)current.direction == i) ? 0 : 1000;

                if (opposites[i][current.direction]) {
                    turn_penalty *= 2;
                }

                int next_cost = gscore[grid_position(&input.grid, current.position)] + turn_penalty + 1;

                // printf("Next: [%d] %d < %d\n", grid_position(&input.grid, next_point), next_cost, gscore[grid_position(&input.grid, next_point)]);

                if (next_cost < gscore[grid_position(&input.grid, next_point)]) {
                    gscore[grid_position(&input.grid, next_point)] = next_cost;
                    fscore[grid_position(&input.grid, next_point)] = next_cost + point_distance(next_point, input.exit);

                    State next_state = (State) {
                        .direction = (Direction)i,
                        .position = next_point,
                        .cost_heuristic = fscore[grid_position(&input.grid, next_point)],
                        .accumulated_cost = current.accumulated_cost + turn_penalty + 1,
                    };

                    pq_push(queue, next_state);
                }

            }
        }
    }

    printf("Part 1: Best cost was %d\n", lowest);
}

int grid_position_direction(Grid *grid, Point position, int direction) {
    return (direction * grid->height * grid->width) + (position.y * grid->width) + position.x;
}

int count_non_max(PriorityQueue *queue) {
    int result = 0;

    for (int i = 0; i < queue->size; i++) {
        if (queue->heap[i].cost_heuristic != INT_MAX) {
            result++;
        }
    }

    return result;
}


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

void set_add(char *set, int bit) {
    int byte_offset = bit / 8;
    int bit_offset = bit % 8;

    set[byte_offset] |= (1 << bit_offset);
}

int set_contains(char *set, int bit) {
    int byte_offset = bit / 8;
    int bit_offset = bit % 8;

    return (set[byte_offset] & (1 << bit_offset)) != 0;
}

void part2() {
    // Hello Dijkstra
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
    int dist[MAX_GRID * 4] = { 0 };

    int best = INT_MAX;

    for (int direction = 0; direction < 4; direction++) {
        for (int row = 0; row < input.grid.height; row++) {
            for (int col = 0; col < input.grid.width; col++) {

                if (grid_get(&input.grid, (Point) { .x = col, .y = row }) == '#') {
                    continue;
                }

                int idx = grid_position_direction(&input.grid, (Point) { .x = col, .y = row }, direction);

                int initial_distance = INT_MAX;

                if (point_equals(input.reindeer, (Point) { .x = col, .y = row }) && ((Direction)direction == input.direction)) {
                    initial_distance = 0;
                }

                dist[idx] = initial_distance;

                State entry = (State) {
                    .accumulated_cost = 1,
                    .cost_heuristic = initial_distance,
                    .direction = (Direction) direction,
                    .position = (Point) { .x = col, .y = row },
                };

                assert((size_t)(input.grid.width * input.grid.height) < sizeof(entry.visited_points) * 8);

                bzero(entry.visited_points, sizeof(entry.visited_points));
                set_add(entry.visited_points, grid_position(&input.grid, (Point) { .x = col, .y = row }));

                pq_push(queue, entry);
            }
        }
    }

    while (pq_len(queue) > 0) {
        State current = pq_pop(queue);

        if (current.cost_heuristic == INT_MAX) {
            break;
        }

        if (point_equals(current.position, input.exit)) {
            if (current.cost_heuristic <= best) {
                printf("Part 2 best path cost: %d\n", current.cost_heuristic);

                int visited = 0;
                for (int i = 0; i < input.grid.width * input.grid.height; i++) {
                    if (set_contains(current.visited_points, i)) {
                        visited++;
                    }
                }

                printf("Part 2 unique Squares visited: %d\n", visited);
                best = current.cost_heuristic;
                continue;
            } else {
                break;
            }
        }

        assert(dist[grid_position_direction(&input.grid, current.position, (int)current.direction)] != INT_MAX);

        for (int i = 0; i < alen(offsets); i++) {
            Point next_point = point_add(current.position, offsets[i]);

            if (grid_get(&input.grid, next_point) == '#') {
                continue;
            }

            int turn_penalty = ((int)current.direction == i) ? 0 : 1000;
            if (opposites[i][current.direction]) {
                turn_penalty *= 2;
            }

            int next_cost = dist[grid_position_direction(&input.grid, current.position, (int)current.direction)] + turn_penalty + 1;
            assert(next_cost > 0);

            if (next_cost <= dist[grid_position_direction(&input.grid, next_point, i)]) {
                int was_equal = next_cost == dist[grid_position_direction(&input.grid, next_point, i)];

                dist[grid_position_direction(&input.grid, next_point, i)] = next_cost;

                State key_state = (State) { .direction = (Direction)i, .position = next_point };
                pq_set_priority(queue, key_state, next_cost);

                State *entry = pq_get(queue, key_state);

                if (was_equal) {
                    // append
                    for (size_t e = 0; e < sizeof(entry->visited_points); e++) {
                        entry->visited_points[e] |= current.visited_points[e];
                    }
                } else {
                    // fully replace
                    memcpy(entry->visited_points, current.visited_points, sizeof(entry->visited_points));
                }

                set_add(entry->visited_points, grid_position(&input.grid, next_point));
            }
        }
    }
}

int main() {
    part1();
    part2();

    return 0;
}
