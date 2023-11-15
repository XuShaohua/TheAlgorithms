// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef LIST__STACK_H_
#define LIST__STACK_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

typedef struct stack_s Stack;
#define element_type int32_t

Stack* stack_new(size_t size);
void stack_free(Stack* stack);

bool stack_is_full(Stack* stack);
bool stack_is_empty(Stack* stack);
size_t stack_size(Stack* stack);

void stack_push(Stack* stack, element_type value);
element_type stack_pop(Stack* stack);
element_type stack_top(Stack* stack);

void stack_debug_print(Stack* stack);

#ifdef __cplusplus
}
#endif

#endif //LIST__STACK_H_
