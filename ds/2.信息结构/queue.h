// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef DS_QUEUE_H
#define DS_QUEUE_H

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct queue_s queue_t;
#define element_type int32_t

queue_t* queue_new(size_t size);
void queue_free(queue_t* queue);

bool queue_is_full(queue_t* queue);
bool queue_is_empty(queue_t* queue);
size_t queue_size(queue_t* queue);

void queue_add(queue_t* queue, element_type value);
element_type queue_delete(queue_t* queue);
element_type queue_get(queue_t* queue);

void queue_debug_print(queue_t* queue);

#ifdef __cplusplus
}
#endif

#endif  // DS_QUEUE_H
