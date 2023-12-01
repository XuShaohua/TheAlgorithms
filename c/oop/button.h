// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#ifndef C_BUTTON_H_
#define C_BUTTON_H_

#include "widget.h"

struct ng_button_s {
#define NG_BUTTON_IMPL \
  NG_WIDGET_IMPL \
  const char* label;
};
typedef struct ng_button_s ng_button_t;
#define NG_BUTTON(button) (ng_button_t*)(button)

extern ng_button_t* ng_button_new();

extern void ng_button_init(ng_button_t* button);

extern void ng_button_destroy(ng_object_t* obj);

#endif  // C_BUTTON_H_
