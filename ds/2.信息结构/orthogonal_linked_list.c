// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include <stdint.h>

typedef struct ol_node_s ol_node_t;
#define element_type int32_t

// 二维十字链表
struct ol_node_s {
  int32_t row;
  int32_t column;
  element_type value;
  ol_node_t* right;
  ol_node_t* down;
};

// 用多重链表表示稀疏矩阵
struct tag_s {

};

struct term_s {
  void* right;
  void* down;
  int32_t row;
  int32_t column;
  element_type value;
};

struct head_s {
  void* right;
  void* down;
};

// 广义表
typedef struct general_list_s general_list_t;
struct general_list_s {
  general_list_t* next;
  element_type value;
  general_list_t* sub_list;
  int32_t has_sub_list;
};