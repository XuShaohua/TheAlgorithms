// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "cii/atom.h"

#include <assert.h>
#include <limits.h>
#include <string.h>
#include <stdlib.h>

#include "cii/hash.h"

struct atom_s {
  struct atom_s* link;
  size_t len;
  char* str;
};

typedef struct atom_s atom_t;

#define ATOM_BUCKET_LEN 2048
static atom_t* g_buckets[ATOM_BUCKET_LEN];

atom_t* atom_new_node(const char* str, size_t len) {
  const size_t total_len = sizeof(atom_t) + len + 1;
  atom_t* p = malloc(total_len);
  assert(p != NULL);
  p->len = len;
  p->str = (char*)(p + 1);
  if (len > 0) {
    memcpy(p->str, str, len);
  }
  p->str[len] = '\0';
  return p;
}

const char* atom_new(const char* str, size_t len) {
  assert(str != NULL);
  size_t hash = str_hash(str, len);
  hash %= ATOM_BUCKET_LEN;

  atom_t* p;
  for (p = g_buckets[hash]; p != NULL; p = p->link) {
    if (len == p->len && strncmp(p->str, str, len) == 0) {
      return p->str;
    }
  }

  // create new entry
  p = atom_new_node(str, len);
  p->link = g_buckets[hash];
  g_buckets[hash] = p;

  return p->str;
}

const char* atom_string(const char* str) {
  assert(str != NULL);
  return atom_new(str, strlen(str));
}

const char* atom_int(int64_t n) {
  uint64_t m;
  if (n == INT64_MIN) {
    m = INT64_MAX + 1UL;
  } else if (n < 0) {
    m = -n;
  } else {
    m = n;
  }

  // 43 characters can hold decimal representation of 128-bit integers.
  const size_t kBufLen = 43;
  char buf[kBufLen];
  char* end_buf = buf + kBufLen;
  char* s = end_buf;
  while (m > 0) {
    s -= 1;
    *s = m % 10 + '0';
    m /= 10;
  }
  if (n < 0) {
    s -= 1;
    *s = '-';
  }
  assert(s >= buf);

  return atom_new(s, end_buf - s);
}


