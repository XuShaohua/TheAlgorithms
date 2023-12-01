// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "cii/array.h"

#include <string.h>

#include "cii/array_rep.h"
#include "cii/assert.h"
#include "cii/mem.h"

struct array_s {
  size_t length;
  size_t element_size;
  char* buffer;
};

array_t* array_new(size_t length, size_t elem_size) {
  array_t* array;
  NEW(array);
  array->length = length;
  array->element_size = elem_size;
  if (length > 0) {
    array->buffer = CALLOC(length, elem_size);
  } else {
    array->buffer = NULL;
  }

  return array;
}

void array_free(array_t** array) {
  assert(array != NULL && *array != NULL);
  FREE((*array)->buffer);
  FREE(array);
}

size_t array_length(array_t* array) {
  assert(array != NULL);
  return array->length;
}

size_t array_element_size(array_t* array) {
  assert(array != NULL);
  return array->element_size;
}