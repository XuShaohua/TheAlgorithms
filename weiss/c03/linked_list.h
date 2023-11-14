// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#ifndef WEISS_C03_LINKED_LIST_H_
#define WEISS_C03_LINKED_LIST_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

// NOTE(Shaohua): node value type in list is defined as integer for simplicity.
typedef int element_type;

struct node_s {
  element_type value;
  struct node_s* next;
};

typedef struct node_s node_t;

/**
 * Check whether list is empty.
 *
 * @param list
 * @return
 */
extern bool list_is_empty(node_t* list);

/**
 * To find if |current| node is the last one in the list.
 *
 * @param list
 * @param current
 * @return
 */
extern bool list_is_last(node_t* list, node_t* current);

/**
 * Find node with |value| in |list|.
 *
 * Returns a null pointer if not found.
 *
 * @param list
 * @param value
 * @return
 */
extern node_t* list_find(node_t* list, element_type value);

/**
 * Find node whose next node is with |value| in |list|.
 *
 * Returns a null pointer if not found.
 *
 * Note that if the head node is with this element value, it also returns NULL,
 * as head node has no previous node.
 *
 * @param list
 * @param value
 * @return
 */
extern node_t* list_find_previous(node_t* list, element_type value);

/**
 * Delete the first node with |value| from list.
 *
 * Returns true if that node is found and deleted.
 *
 * @param list
 * @param value
 * @return
 */
extern bool list_delete(node_t** list, element_type value);

/**
 * Insert a new node with |value| after specific node at |position| into |list|.
 *
 * And returns position of that new node.
 *
 * If |position| is null, insert that new node to head of list.
 *
 * @param list
 * @param position
 * @param value
 * @return
 */
extern node_t* list_insert(node_t** list, node_t* position, element_type value);

/**
 * Calls |apply| function for each node in list.
 *
 * @param list
 * @param apply
 * @return
 */
extern void list_map(node_t* list, void apply(node_t* node, void* user_data),
                     void* user_data);

/**
 * Get number of nodes in list.
 *
 * @param list
 * @return
 */
extern size_t list_length(node_t* list);

/**
 * Print all elements in a list.
 *
 * @param list
 * @return
 */
extern void list_debug_print(node_t* list);

#endif  // WEISS_C03_LINKED_LIST_H_
