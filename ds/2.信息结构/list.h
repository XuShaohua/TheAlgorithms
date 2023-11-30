// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef DS_LIST_H_
#define DS_LIST_H_

#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct list_s list_t;

#define element_type int32_t

list_t* list_new(size_t size);

void list_free(list_t* list);

element_type list_value(list_t* list, size_t position);

void list_set_value(list_t* list, size_t position, element_type new_value);

ssize_t list_index_of(list_t* list, element_type value);

void list_append(list_t* list, element_type value);

void list_prepend(list_t* list, element_type value);

void list_insert(list_t* list, size_t position, element_type value);

void list_delete(list_t* list, size_t position);

size_t list_length(list_t* list);

void list_debug_print(list_t* list);

#ifdef __cplusplus
}
#endif

#endif  // DS_LIST_H_
