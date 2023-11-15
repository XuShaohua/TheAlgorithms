// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "stack.h"

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

struct stack_s {
  size_t length;
  size_t capacity;
  element_type* data;
};

static void stack_resize_expand(Stack* stack, size_t new_size) {
  assert(stack->length < new_size);
  if (new_size < stack->capacity) {
    stack->length = new_size;
  } else {
    const size_t new_capacity = (stack->capacity > new_size) ? (stack->capacity * 2) : (new_size * 2);
    const size_t mem_size = sizeof(element_type) * new_capacity;
    void* ptr = malloc(mem_size);
    assert(ptr != NULL);
    memset(ptr, 0, mem_size);
    memmove(ptr, stack->data, sizeof(element_type) * stack->length);
    stack->data = ptr;
    stack->capacity = new_capacity;
    stack->length = new_size;
  }
}

Stack* stack_new(size_t size) {
  Stack* stack = malloc(sizeof(Stack));
  assert(stack != NULL);
  const size_t mem_size = sizeof(element_type) * size;
  void* ptr = malloc(mem_size);
  assert(ptr != NULL);
  memset(ptr, 0, mem_size);
  stack->length = 0;
  stack->capacity = size;
  stack->data = ptr;
  return stack;
}

void stack_free(Stack* stack) {
  free(stack->data);
  stack->data = NULL;
  free(stack);
}

bool stack_is_full(Stack* stack) {
  return stack->length + 1 == stack->capacity;
}

bool stack_is_empty(Stack* stack) {
  return stack->length == 0;
}

size_t stack_size(Stack* stack) {
  return stack->length;
}

void stack_push(Stack* stack, element_type value) {
  stack_resize_expand(stack, stack->length + 1);
  stack->data[stack->length - 1] = value;
}

element_type stack_pop(Stack* stack) {
  assert(!stack_is_empty(stack));
  stack->length -= 1;
  return stack->data[stack->length];
}

element_type stack_top(Stack* stack) {
  assert(!stack_is_empty(stack));
  return stack->data[stack->length - 1];
}

void stack_debug_print(Stack* stack) {
  for (size_t i = stack->length; i > 0; --i) {
    printf("%d, ", stack->data[i - 1]);
  }
  printf("\n");
}