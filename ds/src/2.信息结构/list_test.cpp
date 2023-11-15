// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include <gtest/gtest.h>

#include "list.h"

TEST(ListTest, TestNew) {
  list_t* list = list_new(4);
  ASSERT_NE(list, nullptr);
  ASSERT_EQ(list_length(list), 4);
  list_free(list);
}

TEST(ListTest, TestGetSet) {
  list_t* list = list_new(4);
  ASSERT_EQ(list_length(list), 4);
  ASSERT_EQ(list_value(list, 2), 0);
  list_set_value(list, 2, 42);
  ASSERT_EQ(list_value(list, 2), 42);
  list_free(list);
}

TEST(ListTest, TestIndexOf) {
  list_t* list = list_new(4);
  list_set_value(list, 2, 42);
  ASSERT_EQ(list_index_of(list, 42), 2);
  ASSERT_EQ(list_index_of(list, -42), -1);
  list_free(list);
}

TEST(ListTest, TestAppend) {
  list_t* list = list_new(4);
  list_set_value(list, 3, 43);
  list_append(list, 44);
  ASSERT_EQ(list_value(list, 3), 43);
  ASSERT_EQ(list_value(list, 4), 44);
  list_free(list);
}

TEST(ListTest, TestPrepend) {
  list_t* list = list_new(4);
  list_set_value(list, 0, 40);
  list_prepend(list, 39);
  ASSERT_EQ(list_value(list, 0), 39);
  ASSERT_EQ(list_value(list, 1), 40);
  list_free(list);
}

TEST(ListTest, TestInsertHead) {
  list_t* list = list_new(4);
  list_set_value(list, 0, 40);
  list_insert(list, 0, 39);
  ASSERT_EQ(list_length(list), 5);
  ASSERT_EQ(list_value(list, 0), 39);
  ASSERT_EQ(list_value(list, 1), 40);
}

TEST(ListTest, TestInsertMiddle) {
  list_t* list = list_new(4);
  list_set_value(list, 1, 41);
  list_set_value(list, 2, 42);
  list_insert(list, 1, 52);
  ASSERT_EQ(list_length(list), 5);
  ASSERT_EQ(list_value(list, 1), 52);
  ASSERT_EQ(list_value(list, 2), 41);
  ASSERT_EQ(list_value(list, 3), 42);
}

TEST(ListTest, TestInsertTail) {
  list_t* list = list_new(4);
  list_set_value(list, 3, 44);
  list_insert(list, 4, 45);
  ASSERT_EQ(list_length(list), 5);
  ASSERT_EQ(list_value(list, 3), 44);
  ASSERT_EQ(list_value(list, 4), 45);
}

TEST(ListTest, TestDeleteHead) {
  list_t* list = list_new(4);
  list_set_value(list, 0, 40);
  list_set_value(list, 1, 41);
  list_delete(list, 0);
  ASSERT_EQ(list_length(list), 3);
  ASSERT_EQ(list_value(list, 0), 41);
  list_free(list);
}

TEST(ListTest, TestDeleteMiddle) {
  list_t* list = list_new(4);
  list_set_value(list, 2, 42);
  list_set_value(list, 3, 43);
  list_delete(list, 2);
  ASSERT_EQ(list_length(list), 3);
  ASSERT_EQ(list_value(list, 2), 43);
  list_free(list);
}

TEST(ListTest, TestDeleteTail) {
  list_t* list = list_new(4);
  list_set_value(list, 2, 42);
  list_set_value(list, 3, 43);
  list_delete(list, 3);
  ASSERT_EQ(list_length(list), 3);
  ASSERT_EQ(list_value(list, 2), 42);
  list_free(list);
}