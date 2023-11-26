// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "cii/assert.h"

#include "cii/except.h"

void (assert)(int e) {
  assert(e);
}

const except_t kAssertFailed = { "Assertion Failed" };