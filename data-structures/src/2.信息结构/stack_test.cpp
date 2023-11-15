// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include <gtest/gtest.h>

#include "stack.h"

TEST(StackTest, StackNew) {
  Stack* stack = stack_new(4);
  ASSERT_TRUE(stack != nullptr);
  stack_free(stack);
}

TEST(StackTest, StackIsEmpty) {
  Stack* stack = stack_new(4);
  ASSERT_TRUE(stack_is_empty(stack));
  stack_free(stack);
}

TEST(StackTest, StackPush) {
  Stack* stack = stack_new(4);
  stack_push(stack, 40);
  stack_push(stack, 41);
  stack_push(stack, 42);
  ASSERT_FALSE(stack_is_empty(stack));
  ASSERT_EQ(stack_size(stack), 3);
  stack_free(stack);
}

TEST(StackTest, StackDebugPrint) {
  Stack* stack = stack_new(4);
  ASSERT_TRUE(stack_is_empty(stack));
  stack_push(stack, 40);
  stack_push(stack, 41);
  stack_push(stack, 42);
  stack_free(stack);
}
