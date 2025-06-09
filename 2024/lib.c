#include <stdint.h>
#include <stdlib.h>
#include <assert.h>
#include <limits.h>

#define alen(a) ((int)(sizeof(a) / sizeof(a[0])))

int intlessthan(const void *a, const void *b) {
    return *(const int *)a - *(const int *)b;
}

uint64_t readint(char *buf, int *offset) {
    uint64_t result = 0;

    while (buf[*offset] >= '0' && buf[*offset] <= '9') {
        result *= 10;
        result += (uint64_t)(buf[*offset] - '0');

        *offset += 1;
    }

    return result;
}

int intcast(uint64_t n) {
    if (n < INT_MAX) {
        return (int) n;
    } else {
        assert(0);
    }
}

int intlog10(uint64_t n) {
    int result = 0;

    while (n > 0) {
        n /= 10;
        result++;
    }

    return result;
}

typedef struct hash_entry {
    void *key;
    void *value;
    struct hash_entry *next;
} hash_entry;

typedef struct hash_table {
    hash_entry **slots;
    uint64_t slot_count;

    uint64_t (*hash_key)(void *key);
    int (*key_cmp)(void *a, void *b);
    void (*free_key)(void *key);
    void (*free_value)(void *value);
} hash_table;


void hash_clear(hash_table *table) {
    for (uint64_t i = 0; i < table->slot_count; i++) {
        hash_entry *entry = table->slots[i];

        while (entry) {
            if (table->free_key) {
                table->free_key(entry->key);
            }

            if (table->free_value) {
                table->free_value(entry->value);
            }

            hash_entry *next = entry->next;
            free(entry);

            entry = next;
        }

        table->slots[i] = NULL;
    }
}

void *set_hash(hash_table *table, void *key, void *value) {
    uint64_t slot_idx = table->hash_key(key) % table->slot_count;

    hash_entry *entry = table->slots[slot_idx];

    if (entry == NULL) {
        entry = calloc(1, sizeof(struct hash_entry));
        entry->key = key;
        entry->value = value;

        table->slots[slot_idx] = entry;

        return NULL;
    }

    for (;;) {
        if (table->key_cmp(entry->key, key) == 0) {
            if (table->free_key) {
                // We didn't use it
                table->free_key(key);
            }

            void *old_value = entry->value;
            entry->value = value;
            return old_value;
        } else {
            if (!entry->next) {
                entry->next = calloc(1, sizeof(struct hash_entry));
                entry->next->key = key;
                entry->next->value = value;

                return NULL;
            }

            entry = entry->next;
        }
    }
}

void *get_hash(hash_table *table, void *key) {
    uint64_t slot_idx = table->hash_key(key) % table->slot_count;

    hash_entry *entry = table->slots[slot_idx];

    if (entry == NULL) {
        return NULL;
    }

    for (;;) {
        if (table->key_cmp(entry->key, key) == 0) {
            return entry->value;
        } else {
            if (!entry->next) {
                return NULL;
            }

            entry = entry->next;
        }
    }
}
