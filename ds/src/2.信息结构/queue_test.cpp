// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include <gtest/gtest.h>

#include "queue.h"

TEST(ArrayQueueTest, TestNew) {
  queue_t* queue = queue_new(5);
  ASSERT_NE(queue, nullptr);
  ASSERT_TRUE(queue_is_empty(queue));
  queue_free(queue);
}

TEST(ArrayQueueTest, TestAdd) {
  queue_t* queue = queue_new(5);
  queue_add(queue, 0);
  ASSERT_EQ(queue_get(queue), 0);
  ASSERT_EQ(queue_size(queue), 1);
  queue_debug_print(queue);
  queue_add(queue, 1);
  ASSERT_EQ(queue_get(queue), 0);
  ASSERT_EQ(queue_size(queue), 2);
  queue_debug_print(queue);
  queue_free(queue);
}

TEST(ArrayQueueTest, TestDelete) {
  queue_t* queue = queue_new(3);
  queue_add(queue, 0);
  queue_add(queue, 1);
  queue_add(queue, 2);
  queue_debug_print(queue);

  queue_delete(queue);
  queue_debug_print(queue);
  ASSERT_EQ(queue_size(queue), 2);

  queue_add(queue, 0);
  ASSERT_EQ(queue_get(queue), 1);
  ASSERT_EQ(queue_size(queue), 3);
  queue_debug_print(queue);
  queue_free(queue);
}