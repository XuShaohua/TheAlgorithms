// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "cii/array.h"

#include <string.h>

#include "cii/arith.h"
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

void* array_get(array_t* array, size_t index) {
  assert(array != NULL);
  assert(index < array->element_size);
  return array->buffer + index * array->element_size;
}

void* array_put(array_t* array, size_t index, void* elem) {
  assert(array != NULL);
  assert(index < array->element_size);
  assert(elem != NULL);
  void* dest = array->buffer + index * array->element_size;
  memcpy(dest, elem, array->element_size);
  return elem;
}

void array_resize(array_t* array, size_t length) {
  assert(array != NULL);
  if (length == 0) {
    FREE(array->buffer);
  } else if (array->length == 0) {
    array->buffer = CALLOC(length, array->element_size);
  } else {
    RESIZE(array->buffer, length * array->element_size);
  }
  array->length = length;
}

array_t* array_copy(array_t* array, size_t length) {
  assert(array != NULL);
  array_t* copy = array_new(length, array->element_size);
  if (length > 0 && array->length > 0) {
    const size_t min_length = arith_min(array->length, length);
    memcpy(copy->buffer, array->buffer, min_length * array->element_size);
  }

  return copy;
}