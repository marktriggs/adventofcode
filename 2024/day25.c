#include <sys/types.h>
#include <unistd.h>

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <stdint.h>

#include "lib.c"

typedef struct {
    int columns[5];
} LockOrKey;


void part1() {
    LockOrKey locks[1024] = { 0 };
    LockOrKey keys[1024] = { 0 };

    int lock_count = 0;
    int key_count = 0;

    FILE *fh = fopen("input_files/day25.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    char buf[4096];

    LockOrKey *current = NULL;

    for (;;) {
        if (fgets(buf, sizeof buf - 1, fh) == NULL) {
            break;
        }

        if (strcmp(buf, "\n") == 0) {
            current = NULL;
            continue;
        }

        if (current == NULL) {
            if (buf[0] == '#') {
                // lock
                current = &locks[lock_count++];
                assert(lock_count < alen(locks));
            } else {
                // key
                current = &keys[key_count++];
                assert(key_count < alen(keys));
            }

        }

        for (int i = 0; i < alen(current->columns); i++) {
            if (buf[i] == '#') {
                current->columns[i]++;
            }
        }
    }

    int fits = 0;

    for (int lock_idx = 0; lock_idx < lock_count; lock_idx++) {
        LockOrKey *lock = &locks[lock_idx];

        for (int key_idx = 0; key_idx < key_count; key_idx++) {
            LockOrKey *key = &keys[key_idx];

            int fit = 1;

            for (int i = 0; i < alen(lock->columns); i++) {
                if (((lock->columns[i] - 1) + (key->columns[i] - 1)) > 5) {
                    fit = 0;
                }
            }

            if (fit) {
                fits++;
            }
        }
    }

    printf("Matched: %d\n", fits);
}

int main() {
    part1();
}
