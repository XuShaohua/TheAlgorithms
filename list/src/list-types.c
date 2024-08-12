// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

// 单链表
struct singly_list_s {
  int value;
  struct singly_list_s* next;
};

// 双链表
struct doubly_list_s {
  int value;
  struct doubly_list_s* previous;
  struct doubly_list_s* next;
};

// 环状链表
struct circular_list_s {
  int value;
  struct circular_list_s* next;
};

// 环状双链表
struct doubly_circular_list_s {
  int value;
  struct doubly_circular_list_s* previous;
  struct doubly_circular_list_s* next;
};

// Header Linked List
struct header_list_s {
  int value;
  struct header_list_s* next;
};