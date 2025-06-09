#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <strings.h>
#include <assert.h>
#include <stdint.h>

#include "lib.c"

typedef struct {
    int storage[131072];
    int count;
} Input;


Input load_input() {
    FILE *fh = fopen("input_files/day9.txt", "r");

    if (fh == NULL) {
        perror("File not found");
    }

    Input result = { 0 };

    int ch = 0;
    int reading_file = 1;
    int file_id = 0;

    while ((ch = fgetc(fh)) != '\n') {
        int size = ch - '0';
        int value = reading_file ? file_id++ : -1;

        for (int i = 0; i < size; i++) {
            result.storage[result.count++] = value;
            assert((size_t)result.count < sizeof(result.storage));
        }

        reading_file = !reading_file;
    }

    fclose(fh);

    return result;
}

void print_input(Input input) {
    for (int i = 0; i < input.count; i++) {
        if (input.storage[i] < 0) {
            printf("[.]");
        } else {
            printf("[%d]", input.storage[i]);
        }
    }
    printf("\n");

}

void part1() {
    Input input = load_input();

    int left_idx = 0;
    int right_idx = input.count - 1;

    while (left_idx < right_idx) {
        while (left_idx < right_idx && input.storage[left_idx] >= 0) {
            left_idx++;
        }

        while (right_idx > left_idx && input.storage[right_idx] < 0) {
            right_idx--;
        }

        if (left_idx >= right_idx) {
            break;
        }

        assert(input.storage[left_idx] < 0);

        input.storage[left_idx] = input.storage[right_idx];
        input.storage[right_idx] = -1;
    }

    uint64_t total = 0;
    for (int position = 0; position < input.count; position++) {
        if (input.storage[position] >= 0) {
            total += (uint64_t)(position * input.storage[position]);
        }
    }

    printf("Part 1 total: %ld\n", total);
}

typedef struct FreeSpaceStruct {
    int length;
    int position;
} FreeSpace;

void part2() {
    Input input = load_input();

    FreeSpace free_list[32768] = { 0 };
    int free_count = 0;

    for (int i = 0; i < input.count; i++) {
        if (input.storage[i] < 0) {
            FreeSpace *space = &free_list[free_count++];
            assert((size_t)free_count < sizeof free_list);

            space->position = i;

            do {
                i++;
                space->length++;
            } while (input.storage[i] < 0 && i < input.count);
        }
    }

    for (int i = input.count - 1; i >= 0;) {
        int value = input.storage[i];
        if (value < 0) {
            i--;
            continue;
        }

        int end_idx = i;
        while (input.storage[i] == value) {
            i--;
        }

        int start_idx = i + 1;

        int length = (end_idx - start_idx) + 1;

        for (int free_idx = 0; free_idx < free_count; free_idx++) {
            FreeSpace *free_slot = &free_list[free_idx];

            if (free_slot->position >= start_idx) {
                break;
            }

            if (free_slot->length >= length) {
                // Place our item here
                for (int j = 0; j < length; j++) {
                    input.storage[free_slot->position + j] = input.storage[start_idx + j];
                    input.storage[start_idx + j] = -1;
                }

                free_slot->length -= length;
                free_slot->position += length;

                assert(free_slot->length >= 0);
                break;
            }
        }
    }

    uint64_t total = 0;
    for (int position = 0; position < input.count; position++) {
        if (input.storage[position] >= 0) {
            total += (uint64_t)(position * input.storage[position]);
        }
    }

    printf("Part 2 total: %ld\n", total);
}


int main() {
    part1();
    part2();

    return 0;
}
