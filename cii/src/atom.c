// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "cii/atom.h"

#include <assert.h>
#include <limits.h>
#include <string.h>

struct atom_s {
  struct atom_s* link;
  size_t len;
  char* str;
};

typedef struct atom_s atom_t;

const size_t kBucketCap = 2048;
static atom_t g_buckets[kBucketCap];

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


