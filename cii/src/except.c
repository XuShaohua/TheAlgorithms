// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "cii/except.h"

#include <assert.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

except_frame_t* g_except_stack = NULL;

void except_raise(const except_t* e, const char* file, int line) {
  except_frame_t* frame = g_except_stack;
  assert(e != NULL);

  if (frame == NULL) {
    // uncaught exception.
    fprintf(stderr, "Uncaught exception");
    if (e->reason != NULL) {
      fprintf(stderr, " %s", e->reason);
    } else {
      fprintf(stderr, " at 0x%p", e);
    }
    if (file != NULL && line > 0) {
      fprintf(stderr, " raised at %s:%d\n", file, line);
    } else {
      fprintf(stderr, "\n");
    }
    fprintf(stderr, "aborting...\n");
    fflush(stderr);
    abort();
  }

  frame->exception = e;
  frame->file = file;
  frame->line = line;
  // pop except frame.
  g_except_stack = frame->prev;
  longjmp(frame->env, except_state_raised);
}