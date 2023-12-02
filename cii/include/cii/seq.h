// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CII_SEQ_H_
#define CII_SEQ_H_

#include <stddef.h>

typedef struct seq_s seq_t;

/**
 * Creates and returns an empty sequence.
 *
 * Sequences expand as necessary to hold their content.
 *
 * May raise kMemFailed.
 *
 * @param size_hint specifies an estimate of the maximum number of values the
 * new sequence will hold. If its value is zero, a small sequence is created.
 * @return
 */
extern seq_t* seq_new(size_t size_hint);

/**
 * Creates and returns a sequence whose value is initialized to its non-null
 * pointer arguments.
 *
 * The argument list is terminated by the first null pointer.
 *
 * @param x
 * @return
 */
extern seq_t* seq_seq(void* x, ...);

/**
 * Deallocates and clears the sequence.
 *
 * @param seq
 */
extern void seq_free(seq_t** seq);

/**
 * Returns number of values in the sequence.
 *
 * @param seq
 * @return
 */
extern size_t seq_length(seq_t* seq);

/**
 * Returns value at |index| in sequence.
 *
 * It is checked runtime error if index is out of range.
 *
 * @param seq
 * @param index
 * @return
 */
extern void* seq_get(seq_t* seq, size_t index);

/**
 * Changes the value at |index| in sequence.
 *
 * It returns |value|.
 *
 * @param seq
 * @param index
 * @param value
 * @return
 */
extern void* seq_put(seq_t* seq, size_t index, void* value);

/**
 * Insert value at the head of sequence.
 *
 * @param seq
 * @param value
 * @return
 */
extern void* seq_push_low(seq_t* seq, void* value);

/**
 * Insert value at the tail of sequence.
 *
 * @param seq
 * @param value
 * @return
 */
extern void* seq_push_high(seq_t* seq, void* value);

/**
 * Removes and returns value at the low end of sequence.
 *
 * It is a checked runtime error if |seq| is empty.
 *
 * @param seq
 * @return
 */
extern void* seq_pop_low(seq_t* seq);

/**
 * Removes and returns value at the high end of sequence.
 *
 * It is a checked runtime error if |seq| is empty.
 *
 * @param seq
 * @return
 */
extern void* seq_pop_high(seq_t* seq);

#endif  // CII_SEQ_H_
