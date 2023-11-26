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
  // Number of key-value pairs.
  size_t length;
  // Size of buckets array.
  size_t size;
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
  table->size = hint_prime;
  table->timestamp = 0;
  table->cmp = (cmp != NULL) ? cmp : cmp_atom;
  table->hash = (hash != NULL) ? hash : hash_atom;
  table->buckets = (struct binding_s**)(table + 1);
  for (size_t i = 0; i < table->size; ++i) {
    table->buckets[i] = NULL;
  }

  return table;
}

size_t table_length(table_t* table) {
  assert(table != NULL);
  return table->length;
}

void* table_put(table_t* table, const void* key, void* value) {
  assert(table != NULL);
  assert(key != NULL);

  // Search table for key.
  struct binding_s* p = NULL;
  size_t index = (*table->hash)(key) % (table->size);
  for (p = table->buckets[index]; p != NULL; p = p->link) {
    if ((*table->cmp)(key, p->key) == 0) {
      break;
    }
  }

  void* prev_value = NULL;
  if (p == NULL) {
    // Allocate a new pair.
    NEW(p);
    p->key = key;
    p->link = table->buckets[index];
    table->buckets[index] = p;
    table->length += 1;
    prev_value = NULL;
  } else {
    prev_value = p->value;
  }
  p->value = value;
  table->timestamp += 1;

  return prev_value;
}

void* table_get(table_t* table, const void* key) {
  assert(table != NULL);
  assert(key != NULL);

  // Search table for key.
  struct binding_s* p = NULL;
  size_t index = (*table->hash)(key) % (table->size);
  for (p = table->buckets[index]; p != NULL; p = p->link) {
    if ((*table->cmp)(key, p->key) == 0) {
      break;
    }
  }

  return (p != NULL) ? p->value : NULL;
}

void table_map(table_t* table,
               void apply(const void* key, void** value, void* user_data),
               void* user_data) {
  assert(table != NULL);
  assert(apply != NULL);

  int64_t timestamp = table->timestamp;
  struct binding_s* p;
  for (size_t i = 0; i < table->size; ++i) {
    for (p = table->buckets[i]; p != NULL; p = p->link) {
      apply(p->key, &p->value, user_data);
      assert(timestamp == table->timestamp);
    }
  }
}