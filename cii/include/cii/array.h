// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CII_ARRAY_H_
#define CII_ARRAY_H_

#include <stddef.h>

typedef struct array_s array_t;

/**
 * Allocates, initializes and returns a new array of |length| elements, each
 * elements occupies |elem_size| bytes.
 *
 * The bytes in each element are initialized to zero.
 *
 * |elem_size| must include any padding that may be required for alignment.
 *
 * May raise kMemFailed.
 *
 * @param length
 * @param elem_size shall be a positive integer.
 * @return
 */
extern array_t* array_new(size_t length, size_t elem_size);

/**
 * Deallocate and clear an array.
 *
 * @param array
 */
extern void array_free(array_t** array);

/**
 * Returns number of elements in |array|.
 *
 * @param array
 * @return
 */
extern size_t array_length(array_t* array);

/**
 * Returns byte size each element occupies in array.
 *
 * @param array
 * @return
 */
extern size_t array_element_size(array_t* array);

/**
 * Returns a pointer to element at |index| in array.
 *
 * Just like arr[index], the subscript version.
 *
 * It is a runtime error if |index| is out of range.
 *
 * @param array
 * @param index
 * @return
 */
extern void* array_get(array_t* array, size_t index);

/**
 * Override value at |index| with the new element pointed at |elem|.
 *
 * It is a runtime error if |index| is out of range.
 *
 * @param array
 * @param index
 * @param elem
 * @return
 */
extern void* array_put(array_t* array, size_t index, void* elem);

/**
 * Change size of array so that it can hold |length| elements.
 *
 * If new length is greater than current value, new elements are added and are
 * initialized to zero.
 *
 * This function may invalid any pointers returned by |array_get|.
 *
 * May raise kMemFailed.
 *
 * @param array
 * @param length
 */
extern void array_resize(array_t* array, size_t length);

/**
 * Returns a copy of array that holds first |length| elements.
 *
 * If new length is greater than current value, new elements are added and are
 * initialized to zero.
 *
 * May raise kMemFailed.
 *
 * @param array
 * @param length
 * @return
 */
extern array_t* array_copy(array_t* array, size_t length);

#endif  // CII_ARRAY_H_
