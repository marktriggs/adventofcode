#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#include "lib.c"

typedef struct {
    int levels[10];
    int count;
} Report;

typedef struct {
    Report reports[2000];
    int count;
} Reports;


typedef struct {
    int success;
    int value;
    int end_position;
    int eof_hit;
} ParsedInt;

typedef enum {
  PART1 = 0,
  PART2 = 1,
} SafetyMode;


ParsedInt read_int(char *s, int offset) {
    int started_read = 0;

    ParsedInt result = { 0 };

    int i;
    for (i = offset; s[i] != '\0'; i++) {
        if (s[i] == ' ' || s[i] == '\n') {
            if (started_read) {
                break;
            } else {
                /* eat leading whitespace */
            }
        } else if (s[i] >= '0' && s[i] <= '9') {
            started_read = 1;

            result.value *= 10;
            result.value += s[i] - '0';
        } else {
            assert(0);
        }
    }

    result.success = started_read;
    result.eof_hit = s[i] == '\0';
    result.end_position = i;

    return result;
}

Reports load_input() {
    FILE *fh = fopen("input_files/day2.txt", "r");
    char buf[4096];

    Reports result = { 0 };

    result.count = 0;
    while (fgets(buf, sizeof(buf) - 1, fh)) {
        int offset = 0;

        for (;;) {
            ParsedInt parsed = read_int(buf, offset);

            if (!parsed.success) {
                break;
            }

            Report *r = &result.reports[result.count];

            r->levels[r->count] = parsed.value;
            r->count++;

            assert(r->count < alen(r->levels));

            offset = parsed.end_position;
        }

        result.count++;
        assert(result.count < alen(result.reports));
    }

    fclose(fh);

    return result;
}

int sign(int value) {
    if (value == 0) {
        return 1;
    } else {
        return value / abs(value);
    }
}

int is_safe(Report report, SafetyMode mode) {
    assert(report.count > 1);

    int max_ignored_idx = ((mode == PART2) ? report.count : 0);

    for (int ignored_level_idx = -1; ignored_level_idx < max_ignored_idx; ignored_level_idx++) {
        int direction = 0;
        int start_idx = 0;

        if (start_idx == ignored_level_idx) {
            start_idx++;
        }

        int end_idx = start_idx + 1;

        if (end_idx == ignored_level_idx) {
            end_idx++;
        }

        while (end_idx < report.count) {
            if (direction != 0 && sign(report.levels[end_idx] - report.levels[start_idx]) != direction) {
                break;
            }

            int difference = report.levels[end_idx] - report.levels[start_idx];

            if (direction == 0) {
                direction = sign(difference);
            }

            if (abs(difference) < 1 || abs(difference) > 3) {
                break;
            }

            start_idx = end_idx;

            do {
                end_idx++;
            } while (end_idx == ignored_level_idx);
        }

        if (end_idx == report.count) {
            return 1;
        }
    }

    return 0;
}

void part1() {
    Reports reports = load_input();

    int safe_count = 0;

    for (int report_idx = 0; report_idx < reports.count; report_idx++) {
        Report report = reports.reports[report_idx];

        if (is_safe(report, PART1)) {
            safe_count++;
        }
    }

    printf("Part 1 safe count: %d\n", safe_count);
}

void part2() {
    Reports reports = load_input();

    int safe_count = 0;

    for (int report_idx = 0; report_idx < reports.count; report_idx++) {
        Report report = reports.reports[report_idx];

        if (is_safe(report, PART2)) {
            safe_count++;
        }

    }

    printf("Part 2 safe count: %d\n", safe_count);
}

int main() {
    part1();
    part2();

    return 0;
}
