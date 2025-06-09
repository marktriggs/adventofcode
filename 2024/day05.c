#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#include "lib.c"

#define MAX_PAGE 100
#define MAX_UPDATES 1024

#define MAX_PAGE_PER_UPDATE 128

typedef struct {
    int members[MAX_PAGE];
} PageSet;

typedef struct {
    int pages[MAX_PAGE_PER_UPDATE];
    int count;
} Update;

typedef struct {
    struct {
        int dependencies[MAX_PAGE];
    } pages[MAX_PAGE];

    Update updates[MAX_UPDATES];
    int update_count;
} Input;

Input load_input() {
    FILE *fh = fopen("input_files/day5.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    Input result = { 0 };

    char buf[4096];

    // Read rules
    for (;;) {
        fgets(buf, sizeof buf - 1, fh);

        if (strcmp(buf, "\n") == 0) {
            break;
        }

        int readpos = 0;
        int a = intcast(readint(buf, &readpos));

        // Skip |
        readpos++;

        int b = intcast(readint(buf, &readpos));

        result.pages[b].dependencies[a] = 1;
    }

    // Read updates
    for (;;) {
        if (!fgets(buf, sizeof buf - 1, fh)) {
            break;
        }

        Update *this_update = &result.updates[result.update_count++];
        assert(result.update_count < MAX_UPDATES);

        // Read our list of page numbers
        int readpos = 0;
        for (;;) {
            int n = intcast(readint(buf, &readpos));

            this_update->pages[this_update->count++] = n;
            assert(this_update->count < MAX_PAGE_PER_UPDATE);

            if (buf[readpos] == '\n') {
                break;
            } else {
                // Skip ,
                readpos++;
            }
        }
    }

    fclose(fh);
    return result;
}


PageSet pageset_for_update(Update *update) {
    PageSet update_pages = { 0 };

    // Build the set of pages in this update
    for (int page_idx = 0; page_idx < update->count; page_idx++) {
        int page = update->pages[page_idx];
        update_pages.members[page] = 1;
    }

    return update_pages;
}

int update_ok(Input *input, Update *update) {
    PageSet update_pages = pageset_for_update(update);

    PageSet seen_pages = { 0 };
    for (int page_idx = 0; page_idx < update->count; page_idx++) {
        int this_page = update->pages[page_idx];

        for (int dep = 0; dep < MAX_PAGE; dep++) {
            if (!update_pages.members[dep]) {
                // This page isn't in this update set, so not relevant
                continue;
            }

            if (input->pages[this_page].dependencies[dep] && !seen_pages.members[dep]) {
                // Bad!
                return 0;
            }
        }

        seen_pages.members[this_page] = 1;
    }

    return 1;
}

void part1() {
    Input input = load_input();

    int middle_page_tally = 0;

    for (int update_idx = 0; update_idx < input.update_count; update_idx++) {
        Update *this_update = &input.updates[update_idx];

        if (update_ok(&input, this_update)) {
            middle_page_tally += this_update->pages[this_update->count / 2];
        }
    }

    printf("Part 1 tally of middle pages: %d\n", middle_page_tally);
}

void part2() {
    Input input = load_input();

    int middle_page_tally = 0;

    for (int update_idx = 0; update_idx < input.update_count; update_idx++) {
        Update *this_update = &input.updates[update_idx];

        if (update_ok(&input, this_update)) {
            continue;
        }

        PageSet update_pages = pageset_for_update(this_update);

        int correct_idx = 0;
        int correct_ordering[MAX_PAGE_PER_UPDATE] = { 0 };
        PageSet seen_pages = { 0 };

        while (correct_idx < this_update->count) {
            // Find the next item with all dependencies satisfied
            for (int page_idx = 0; page_idx < this_update->count; page_idx++) {
                int this_page = this_update->pages[page_idx];

                if (seen_pages.members[this_page]) {
                    continue;
                }

                int all_deps_satisfied = 1;

                for (int dep = 0; dep < MAX_PAGE; dep++) {
                    if (!update_pages.members[dep]) {
                        continue;
                    }

                    // this_page depends on dep, and dep is in this update, but hasn't been encountered yet
                    if (input.pages[this_page].dependencies[dep] && !seen_pages.members[dep]) {
                        all_deps_satisfied = 0;
                        break;
                    }
                }

                if (all_deps_satisfied) {
                    correct_ordering[correct_idx++] = this_page;
                    seen_pages.members[this_page] = 1;
                }
            }
        }

        middle_page_tally += correct_ordering[correct_idx / 2];
    }

    printf("Part 2 tally of middle pages: %d\n", middle_page_tally);
}


int main() {
    part1();
    part2();

    return 0;
}
