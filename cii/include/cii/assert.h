// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CII_ASSERT_H_
#define CII_ASSERT_H_

#undef assert
#ifdef NDEBUG
#define assert(e) ((void)0)
#else

#include "cii/except.h"

extern void assert(int e);

extern const except_t kAssertFailed;

#define assert(e) ((void)((e) || (RAISE(kAssertFailed), 0)))

#endif  // NDEBUG

#endif  // CII_ASSERT_H_
