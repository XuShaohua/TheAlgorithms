// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "get_word.h"

#include <ctype.h>
#include <string.h>

#include "cii/assert.h"

int get_word(FILE* fp, char* buf, size_t buf_size,
             int first(int c), int rest(int c)) {
  assert(fp != NULL);
  assert(buf != NULL);
  assert(buf_size > 1);
  assert(first != NULL);
  assert(rest != NULL);

  int c = getc(fp);
  int i = 0;
  for (/* empty */; c != EOF; c = getc(fp)) {
    if (first(c) == 0) {
      // store c in buffer.
      if (i < buf_size - 1) {
        buf[i] = (char)c;
        i += 1;
      }
      
      c = getc(fp);
      break;
    }
  }
  for (/* empty */; c != EOF && rest(c) == 0; c = getc(fp)) {
    // store c in buffer.
    if (i < buf_size - 1) {
      buf[i] = (char)c;
      i += 1;
    }
  }
  if (i < buf_size) {
    buf[i] = '\0';
  } else {
    buf[buf_size - 1] = '\0';
  }
  if (c != EOF) {
    ungetc(c, fp);
  }

  return i > 0;
}