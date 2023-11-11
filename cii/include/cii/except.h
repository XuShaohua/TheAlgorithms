// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CII_EXCEPT_H_
#define CII_EXCEPT_H_

#include <setjmp.h>

struct except_s {
  const char* reason;
};

typedef struct except_s except_t;

struct except_frame_s {
  struct except_frame_s* prev;
  jmp_buf env;
  const char* file;
  int line;
  const except_t* exception;
};

typedef struct except_frame_s except_frame_t;

// Global exception stack frame.
extern except_frame_t* g_except_stack;

enum except_state_e {
  except_state_entered = 0,
  except_state_raised,
  except_state_handled,
  except_state_finalized,
};

/**
 * It is a checked runtime error if e is null.
 *
 * @param e is a pointer to global or static except_t instance.
 * @param file source file name where exception occurs.
 * @param line line number where exception occurs.
 */
void except_raise(const except_t* e, const char* file, int line);

#define RAISE(e) except_raise(&(e), __FILE__, __LINE__)

#define REREAISE \
  except_raise(except_frame.exception, except_frame.file, except_frame.line)

#define RETURN switch(0) default: return

#define TRY                                    \
do {                                           \
  volatile except_state_e except_flag;         \
  except_frame_t except_frame;                 \
  except_frame.prev = g_except_stack;          \
  g_except_stack = &except_frame;              \
  except_flag = setjmp(except_frame.env);      \
  if (except_flag == except_state_entered) {   \

#define EXCEPT(e)                              \
    if (except_flag == except_state_entered) { \
      g_except_stack = except_stack->prev;     \
    }                                          \
  } else if (except_frame.exception == &(e)) { \
    except_flag = except_state_handled;

#define ELSE                                   \
    if (except_flag == except_state_entered) { \
      g_except_stack = except_stack->prev;     \
    }                                          \
  } else {                                     \
    except_flag = except_state_handled;

#define FINALLY                                \
    if (except_flag == except_state_entered) { \
      g_except_stack = except_stack->prev;     \
    }                                          \
  }                                            \
  {                                            \
    if (except_flag == except_state_entered) { \
      except_flag = except_state_finalized;    \
    }

#define END_TRY                                \
    if (except_flag == except_state_entered) { \
      g_except_stack = except_stack->prev;     \
    }                                          \
  }                                            \
  if (except_flag == except_raised) {          \
    RERAISE;                                   \
  }                                            \
} while (0)

#endif  // CII_EXCEPT_H_
