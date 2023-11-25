// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "cii/table.h"

#include "cii/assert.h"
#include "cii/mem.h"

static int cmp_atom(const void* x, const void* y) {
  return x != y;
}

static size_t hash_atom(const void* key) {
  return (unsigned long)key >> 2;
}

struct binding_s {
  struct binding_s* link;
  const void* key;
  void* value;
};

struct table_s {
  size_t length;
  size_t capacity;
  int64_t timestamp;
  int (*cmp)(const void* x, const void* y);
  size_t (*hash)(const void* key);

  // buckets points to an array with appropriate number of elements.
  struct binding_s** buckets;
};

table_t* table_new(size_t hint,
                  int cmp(const void* x, const void* y),
                  size_t hash(const void* key)) {
  const size_t primes[] = {
    509, 509, 1021, 2053, 4093,
    8191, 16381, 32771, 65521, INT64_MAX
  };
  size_t hint_prime = 0;
  for (int i = 1; primes[i] < hint; ++i) {
    hint_prime = primes[i - 1];
  }
  table_t* table = ALLOC(sizeof(table_t) + hint_prime * sizeof(struct binding_s));
  assert(table != NULL);
  table->length = 0;
  table->capacity = hint_prime;
  table->timestamp = 0;
  table->cmp = (cmp != NULL) ? cmp : cmp_atom;
  table->hash = (hash != NULL) ? hash : hash_atom;
  table->buckets = (struct binding_s**)(table + 1);
  for (size_t i = 0; i < table->capacity; ++i) {
    table->buckets[i] = NULL;
  }

  return table;
}

size_t table_length(table_t* table) {
  return table->length;
}

void* table_put(table_t* table, const void* key, void* value) {

}

void* table_get(table_t* table, const void* key) {
  
}