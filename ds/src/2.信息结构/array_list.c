// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "list.h"

#include <assert.h>
#include <stdio.h>
#include <string.h>

struct list_s {
  size_t capacity;
  size_t length;
  element_type* data;
};

static int32_t list_resize_shrink(list_t* list, size_t new_size) {
  assert(list->length > new_size);
  list->length = new_size;
  // TODO(Shaohua): Free memory if too many elements are unused.
  return 0;
}

static int32_t list_resize_expand(list_t* list, size_t new_size) {
  assert(list->length < new_size);
  if (list->length + new_size < list->capacity) {
    list->length += new_size;
    return 0;
  }

  const size_t new_capacity = (list->capacity > new_size) ? (list->capacity * 2) : (new_size * 2);
  const size_t new_mem_size = sizeof(element_type) * new_capacity;
  void* ptr = malloc(new_mem_size);
  if (ptr == NULL) {
    perror("malloc()");
    return -1;
  }
  memset(ptr, 0, new_mem_size);

  memcpy(ptr, list->data, sizeof(element_type) * list->length);
  list->data = ptr;
  list->length = new_size;
  list->capacity = new_capacity;
  return 0;
}

static int32_t list_resize(list_t* list, size_t new_size) {
  if (new_size == list->length) {
    return 0;
  }
  if (new_size < list->length) {
    return list_resize_shrink(list, new_size);
  }
  return list_resize_expand(list, new_size);
}

list_t* list_new(size_t size) {
  list_t* list = malloc(sizeof(list_t));
  assert(list != NULL);
  const size_t mem_size = sizeof(element_type) * size;
  list->data = malloc(mem_size);
  assert(list->data != NULL);
  memset(list->data, 0, mem_size);
  list->capacity = size;
  list->length = size;
  return list;
}

void list_free(list_t* list) {
  list->length = 0;
  list->capacity = 0;
  free(list->data);
  list->data = NULL;
}

element_type list_value(list_t* list, size_t position) {
  assert(position < list->length);
  if (position >= list->length) {
    fprintf(stderr, "[%s] Index out of range: %ld, len: %ld\n", __func__, position, list->length);
    return 0;
  }
  return list->data[position];
}

void list_set_value(list_t* list, size_t position, element_type new_value) {
  assert(position < list->length);
  if (position >= list->length) {
    fprintf(stderr, "[%s] Index out of range: %ld, len: %ld\n", __func__, position, list->length);
    return;
  }
  list->data[position] = new_value;
}

ssize_t list_index_of(list_t* list, element_type value) {
  for (size_t i = 0; i < list->length; ++i) {
    if (list->data[i] == value) {
      return (ssize_t)i;
    }
  }
  return -1;
}

void list_append(list_t* list, element_type value) {
  if (list_resize_expand(list, list->length + 1) != 0) {
    return;
  }
  list->data[list->length - 1] = value;
}

void list_prepend(list_t* list, element_type value) {
  if (list_resize_expand(list, list->length + 1) != 0) {
    return;
  }
  memmove(list->data + 1, list->data, sizeof(element_type) * list->length - 1);
  list->data[0] = value;
}

void list_insert(list_t* list, size_t position, element_type value) {
  if (list_resize_expand(list, list->length + 1) != 0) {
    return;
  }
  if (position == list->length) {
    // Last element.
    list->data[position] = value;
  } else {
    const size_t move_elems = list->length - position;
    memmove(list->data + position + 1, list->data + position, sizeof(element_type) * move_elems);
    list->data[position] = value;
  }
}

void list_delete(list_t* list, size_t position) {
  assert(position < list->length);
  if (position >= list->length) {
    fprintf(stderr, "[%s] Index out of range: %ld, len: %ld", __func__, position, list->length);
    return;
  }
  if (position < list->length - 1) {
    memmove(list->data + position, list->data + position + 1, 1);
  }
  list->length -= 1;
}

size_t list_length(list_t* list) {
  return list->length;
}

void list_debug_print(list_t* list) {
  assert(list != NULL);
  for (size_t i = 0; i < list->length - 1; ++i) {
    printf("%d, ", list->data[i]);
  }
  if (list->length > 0) {
    printf("%d\n", list->data[list->length - 1]);
  }
}
