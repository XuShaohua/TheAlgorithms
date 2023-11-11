// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CII_ASSERT_H_
#define CII_ASSERT_H_

#include <stdlib.h>
#include <stdio.h>

#undef assert
#ifdef NDEBUG
#define assert(e) ((void)0)
#else

extern void assert(int e);

#define assert(e)                                      \
  do {                                                 \
    if (!(e)) {                                        \
      fprintf(stderr, "%s:%d: Assertion failed: %s\n", \
              __FILE__, (int)__LINE__, #e);            \
      abort();                                         \
  } while(0)

#endif  // NDEBUG

#endif  // CII_ASSERT_H_
