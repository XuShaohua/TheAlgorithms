// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef DATA_STRUCTURES_3_TREE_BINARY_TREE_H_
#define DATA_STRUCTURES_3_TREE_BINARY_TREE_H_

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct binary_tree_s binary_tree_t;

#define element_type int32_t

binary_tree_t* binary_tree_new();
void binary_tree_free(binary_tree_t* tree);

#ifdef __cplusplus
}
#endif

#endif  // DATA_STRUCTURES_3_TREE_BINARY_TREE_H_
