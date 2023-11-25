// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CII_TESTS_GET_WORD_H_
#define CII_TESTS_GET_WORD_H_

#include <stdio.h>

/**
 * Consumes the next word in the file opened on |fp|, stores it as a null-terminated
 * string in |buf|, and returns one.
 *
 * When it reaches end of file without consuming a word, it returns zero.
 *
 * A word is a contiguous sequence of characters, it starts with a character for
 * which |first| returns nonzero value followed by characters for which |rest|
 * returns nonzero values.
 *
 * If a word is longer than |buf_size - 2| characters, the excess characters are
 * discarded.
 * @param fp must be non-null.
 * @param buf must be non-null.
 * @param buf_size must exceed one.
 * @param first must be non-null.
 * @param rest must be non-null.
 * @return
 */
extern int get_word(FILE* fp, char* buf, size_t buf_size,
                    int first(int c), int rest(int c));

#endif //CII_TESTS_GET_WORD_H_
