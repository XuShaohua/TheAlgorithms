// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CII_LIST_H_
#define CII_LIST_H_

#include <stddef.h>

/**
 * Impl single linked list.
 */

struct list_s {
  struct list_s* rest;
  void* value;
};
typedef struct list_s list_t;

/**
 * Append/concat one list to another.
 *
 * If assigns |tail| to the last |rest| field in list.
 *
 * If |list| is null, it returns |tail| list.
 * @param list
 * @param tail
 * @return
 */
extern list_t* list_append(list_t* list, list_t* tail);

/**
 * Makes a copy of |list|.
 * @param list
 * @return
 */
extern list_t* list_copy(list_t* list);

/**
 * Creates and returns a list.
 *
 * It's called with N non-null pointers followed by one null pointer, and it
 * creates a list with N nodes.
 * @param value
 * @param ...
 * @return
 */
extern list_t* list_list(void* value, ...);

/**
 * Assigns the value field of the first node in the list to |*value|, if |*value|
 * is not null.
 *
 * Removes and deallocates the first list node and returns the remaining list.
 *
 * If the list itself is empty, |*value| is not changed.
 * @param list
 * @param value
 * @return
 */
extern list_t* list_pop(list_t* list, void** value);

/**
 * Add a new node that holds |value| to the beginning of the list, and returns
 * the new list.
 * @param list
 * @param value
 * @return
 */
extern list_t* list_push(list_t* list, void* value);

/**
 * Reverses the order of nodes in |list| and returns the resulting list.
 * @param list
 * @return
 */
extern list_t* list_reverse(list_t* list);

/**
 * Walks down list counting its nodes and returns size of nodes in list.
 * @param list
 * @return
 */
extern size_t list_length(list_t* list);

/**
 * If |list| is a non-null pointer Deallocates all nodes in it and set its value
 * to null.
 * @param list
 */
extern void list_free(list_t** list);

/**
 * Call the |apply| function for every node in list.
 * @param list
 * @param apply
 * @param cl
 */
extern void list_map(list_t* list, void apply(void** value, void* cl), void* cl);

/**
 * Create an array to hold N + 1 node values in list and returns that array.
 *
 * The value of N node is set to |end|.
 *
 * @param list
 * @param end
 * @return
 */
extern void** list_to_array(list_t* list, void* end);

#endif  // CII_LIST_H_
