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

// NOTE: matches offsets order below
typedef enum {
    North,
    East,
    South,
    West
} Direction;

typedef struct {
    int x;
    int y;
} Point;

static Point NorthOffset = (Point) { .x = 0, .y = -1 };
static Point EastOffset = (Point) { .x = 1, .y = 0 };
static Point SouthOffset = (Point) { .x = 0, .y = 1 };
static Point WestOffset = (Point) { .x = -1, .y = 0 };

typedef struct {
    int items[MAX_GRID];
    int width;
    int height;
} Grid;

#define MAX_CORRUPTIONS 4096

typedef struct {
    Point corruptions[MAX_CORRUPTIONS];
    int count;
} Input;


Input load_input() {
    FILE *fh = fopen("input_files/day18.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    Input result = { 0 };

    char buf[4096];

    // Read grid
    while (fgets(buf, sizeof(buf) - 1, fh)) {
        int offset = 0;
        int x = (int)readint(buf, &offset);
        offset++;
        int y = (int)readint(buf, &offset);

        result.corruptions[result.count++] = (Point) { .x = x, .y = y };
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

#define MAX_PQ_SIZE 131072

typedef struct {
    Point position;
    int priority;
} State;


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
    // Hello again, Dijkstra
    Input input = load_input();

    Point offsets[4];

    offsets[0] = NorthOffset;
    offsets[1] = EastOffset;
    offsets[2] = SouthOffset;
    offsets[3] = WestOffset;

    PriorityQueue *queue = malloc(sizeof(PriorityQueue));
    int dist[MAX_GRID * 4] = { 0 };

    int grid_width = 71;
    int grid_height = 71;

    int grid[grid_width * grid_height];
    for (int i = 0; i < alen(grid); i++) {
        grid[i] = 0;
    }

    for (int t = 0; t < 1024; t++) {
        grid[(input.corruptions[t].y * grid_width) + input.corruptions[t].x] = 1;
    }

    for (int row = 0; row < grid_height; row++) {
        for (int col = 0; col < grid_width; col++) {
            int idx = (row * grid_width) + col;

            if (grid[idx]) {
                // corrupted
                continue;
            }

            int initial_distance = (row == 0 && col == 0) ? 0 : INT_MAX;

            dist[idx] = initial_distance;

            State entry = (State) {
                .priority = initial_distance,
                .position = (Point) { .x = col, .y = row },
            };

            pq_push(queue, entry);
        }
    }

    Point exit_position = (Point) { .x = 70, .y = 70 };

    while (pq_len(queue) > 0) {
        State current = pq_pop(queue);

        if (current.priority == INT_MAX) {
            break;
        }

        if (point_equals(current.position, exit_position)) {
            printf("Shortest path to exit: %d\n", current.priority);
            break;
        }

        int current_idx = (current.position.y * grid_width) + current.position.x;

        for (int i = 0; i < alen(offsets); i++) {
            Point next_point = point_add(current.position, offsets[i]);

            if (next_point.x < 0 || next_point.y < 0 ||
                next_point.x >= grid_width || next_point.y >= grid_height) {
                // Out of bounds
                continue;
            }

            int next_idx = (next_point.y * grid_width) + next_point.x;

            if (grid[next_idx] == 1) {
                // corrupted
                continue;
            }

            int next_cost = dist[current_idx] + 1;
            assert(next_cost > 0);

            if (next_cost < dist[next_idx]) {
                dist[next_idx] = next_cost;

                State key_state = (State) { .position = next_point };
                pq_set_priority(queue, key_state, next_cost);
            }
        }
    }
}

void part2() {
    // Hello again, Dijkstra
    Input input = load_input();

    Point offsets[4];

    offsets[0] = NorthOffset;
    offsets[1] = EastOffset;
    offsets[2] = SouthOffset;
    offsets[3] = WestOffset;

    PriorityQueue *queue = malloc(sizeof(PriorityQueue));
    int dist[MAX_GRID * 4] = { 0 };

    int grid_width = 71;
    int grid_height = 71;

    int grid[grid_width * grid_height];

    // int lower = 1024;
    // int upper = 4096;

    for (int uppert = 1024; uppert < 4096; uppert++) {
        queue->size = 0;

        for (int i = 0; i < alen(grid); i++) {
            grid[i] = 0;
        }

        for (int t = 0; t < uppert; t++) {
            grid[(input.corruptions[t].y * grid_width) + input.corruptions[t].x] = 1;
        }

        for (int row = 0; row < grid_height; row++) {
            for (int col = 0; col < grid_width; col++) {
                int idx = (row * grid_width) + col;

                if (grid[idx]) {
                    // corrupted
                    continue;
                }

                int initial_distance = (row == 0 && col == 0) ? 0 : INT_MAX;

                dist[idx] = initial_distance;

                State entry = (State) {
                    .priority = initial_distance,
                    .position = (Point) { .x = col, .y = row },
                };

                pq_push(queue, entry);
            }
        }

        Point exit_position = (Point) { .x = 70, .y = 70 };

        while (pq_len(queue) > 0) {
            State current = pq_pop(queue);

            if (current.priority == INT_MAX) {
                printf("NO SOLUTION FOR TIME STEP %d\n", uppert);
                printf("Last corruption: %d,%d\n", input.corruptions[uppert - 1].x, input.corruptions[uppert - 1].y);
                break;
            }

            if (point_equals(current.position, exit_position)) {
                printf("Shortest path to exit after time step %d: %d\n", uppert, current.priority);
                break;
            }

            int current_idx = (current.position.y * grid_width) + current.position.x;

            for (int i = 0; i < alen(offsets); i++) {
                Point next_point = point_add(current.position, offsets[i]);

                if (next_point.x < 0 || next_point.y < 0 ||
                    next_point.x >= grid_width || next_point.y >= grid_height) {
                    // Out of bounds
                    continue;
                }

                int next_idx = (next_point.y * grid_width) + next_point.x;

                if (grid[next_idx] == 1) {
                    // corrupted
                    continue;
                }

                int next_cost = dist[current_idx] + 1;

                assert(next_cost > 0);

                if (next_cost < dist[next_idx]) {
                    dist[next_idx] = next_cost;

                    State key_state = (State) { .position = next_point };
                    pq_set_priority(queue, key_state, next_cost);
                }
            }
        }
    }
}

int main() {
    part1();
    part2();

    return 0;
}
