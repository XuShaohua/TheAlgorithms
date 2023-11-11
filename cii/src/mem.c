// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "cii/mem.h"

#include <stdlib.h>
#include <stddef.h>

#include "cii/assert.h"
#include "cii/except.h"

const except_t kMemFailed = { "Allocation Failed" };

void* mem_alloc(size_t nbytes, const char* file, int line) {
  assert(nbytes > 0);

  void* ptr = malloc(nbytes);
  if (ptr == NULL) {
    if (file == NULL) {
      RAISE(kMemFailed);
    } else {
      except_raise(&kMemFailed, file, line);
    }
  }

  return ptr;
}

void* mem_calloc(size_t count, size_t nbytes, const char* file, int line) {
  assert(count > 0);
  assert(nbytes > 0);
  void* ptr = calloc(count, nbytes);
  if (ptr == NULL) {
    if (file == NULL) {
      RAISE(kMemFailed);
    } else {
      except_raise(&kMemFailed, file, line);
    }
  }

  return ptr;
}

void mem_free(void* ptr, const char* file, int line) {
  if (ptr != NULL) {
    free(ptr);
  }
}

void* mem_resize(void* ptr, size_t nbytes, const char* file, int line) {
  assert(ptr != NULL);
  assert(nbytes > 0);

  ptr = realloc(ptr, nbytes);
  if (ptr == NULL) {
    if (file == NULL) {
      RAISE(kMemFailed);
    } else {
      except_raise(&kMemFailed, file, line);
    }
  }
  return ptr;
}