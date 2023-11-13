// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "cii/list.h"

#include <stdarg.h>
#include <stddef.h>

#include "cii/assert.h"
#include "cii/mem.h"

list_t* list_push(list_t* list, void* value) {
  list_t* p;
  NEW(p);
  p->value = value;
  p->rest = list;
  return p;
}

list_t* list_list(void* value, ...) {
  list_t* list = NULL;
  list_t** p = &list;
  va_list va;

  va_start(va, value);
  for (/* empty */; value != NULL; value = va_arg(va, void*)) {
    NEW(*p);
    (*p)->value = value;
    p = &(*p)->rest;
  }
  (*p) = NULL;

  va_end(va);
  return list;
}

list_t* list_append(list_t* list, list_t* tail) {
  list_t** p = &list;
  while ((*p) != NULL) {
    // Go to tail node of |list|.
    p = &(*p)->rest;
  }
  *p = tail;
  return list;
}

list_t* list_copy(list_t* list) {
  list_t* head = NULL;
  list_t** p = &head;
  for (/* empty */; list != NULL; list = list->rest) {
    NEW(*p);
    (*p)->value = list->value;
    p = &(*p)->rest;
  }

  *p = NULL;
  return head;
}

list_t* list_pop(list_t* list, void** value) {
  if (list != NULL) {
    list_t* head = list->rest;
    if (value != NULL) {
      *value = list->value;
    }
    FREE(list);
    return head;
  } else {
    return list;
  }
}

list_t* list_reverse(list_t* list) {
  list_t* head = NULL;
  list_t* next = NULL;
  for (/* empty */; list != NULL; list = next) {
    next = list->rest;
    list->rest = head;
    head = list;
  }

  return head;
}

size_t list_length(list_t* list) {
  size_t length = 0;
  while (list != NULL) {
    length += 1;
    list = list->rest;
  }

  return length;
}

void list_free(list_t** list) {
  assert(list != NULL);
  list_t* next = NULL;

  while (*list != NULL) {
    next = (*list)->rest;
    FREE(*list);
    *list = next;
  }
}

void list_map(list_t* list, void apply(void** value, void* cl), void* cl) {
  assert(apply != NULL);
  while (list != NULL) {
    apply(&list->value, cl);
    list = list->rest;
  }
}

void** list_to_array(list_t* list, void* end) {
  const size_t len = list_length(list);
  void** array = ALLOC((len + 1) * sizeof(*array));

  size_t i = 0;
  for (/* empty */; i < len; ++i) {
    array[i] = list->value;
    list = list->rest;
  }
  array[i] = end;

  return array;
}