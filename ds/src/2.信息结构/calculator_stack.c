// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "stack.h"

const char kPlus = '+';
const char kMinus = '-';
const char kMultiply = '*';
const char kDivide = '/';

bool has_higher_priority(char a, char b) {
  if ((a == kMultiply || a == kDivide) && (b == kPlus || b == kMinus)) {
    return true;
  }
  return false;
}

int main(void) {
  const char* expression = "2+9/3-5";
  fprintf(stderr, "expression: %s\n", expression);
  fflush(stderr);
  Stack* stack = stack_new(64);
  const size_t exp_len = strlen(expression);
  for (size_t i = 0; i < exp_len; ++i) {
    char c = expression[i];
    if (c >= '0' && c <= '9') {
      fprintf(stderr, "%c ", c);
    } else if (c == kMultiply || c == kDivide || c == kPlus || c == kMinus) {
      if (stack_is_empty(stack) || has_higher_priority(c, (char) stack_top(stack))) {
        stack_push(stack, c);
      } else {
        while (!stack_is_empty(stack) && !has_higher_priority(c, (char)stack_top(stack))) {
          fprintf(stderr, "%c ", (char)stack_pop(stack));
        }
        stack_push(stack, c);
      }
    } else {
      fprintf(stderr, "Invalid char: %c\n", c);
      break;
    }
  }
  while (!stack_is_empty(stack)) {
    const char last_value = (char) stack_pop(stack);
    fprintf(stderr, "%c ", last_value);
  }
  fprintf(stderr, "\n");

  stack_free(stack);
  return 0;
}