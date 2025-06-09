#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#include "lib.c"

typedef struct {
    int alist[1000];
    int blist[1000];
    int max;
    size_t count;
} input;

input load_input() {
    FILE *fh = fopen("input_files/day1.txt", "r");
    char buf[4096];

    input result;

    result.max = 0;
    result.count = 0;
    while (fgets(buf, sizeof(buf) - 1, fh)) {
        int a, b;

        sscanf(buf, "%d %d", &a, &b);

        assert(result.count < alen(result.alist));
        assert(result.count < alen(result.blist));

        result.alist[result.count] = a;
        result.blist[result.count] = b;

        if (a > result.max) { result.max = a; }
        if (b > result.max) { result.max = b; }

        result.count++;
    }

    qsort(result.alist, result.count, sizeof(int), intlessthan);
    qsort(result.blist, result.count, sizeof(int), intlessthan);

    fclose(fh);

    return result;
}

void part1() {
    input input = load_input();

    int total = 0;

    for (size_t i = 0; i < input.count; i++) {
        total += abs(input.alist[i] - input.blist[i]);
    }

    printf("Part 1 total: %d\n", total);
}

void part2() {
    input input = load_input();

    int value_counts[input.max + 1];
    memset(value_counts, 0, sizeof(value_counts));

    for (size_t i = 0; i < input.count; i++) {
        value_counts[input.blist[i]]++;
    }

    int result = 0;

    for (size_t i = 0; i < input.count; i++) {
        result += input.alist[i] * value_counts[input.alist[i]];
    }

    printf("Part 2 total: %d\n", result);
}

int main() {
    part1();
    part2();

    return 0;
}
