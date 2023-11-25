// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CII_TESTS_WF_H_
#define CII_TESTS_WF_H_

#include <stdio.h>

/**
 * wf uses a table to store words and their counts.
 *
 * Each word is folded to lowercase, converted to an atom, and used as a key.
 * @param filename
 * @param fp
 */
extern void wf(const char* filename, FILE* fp);

extern int first(int c);

extern int rest(int c);

extern int compare(const void* x, const void* y);

#endif //CII_TESTS_WF_H_
