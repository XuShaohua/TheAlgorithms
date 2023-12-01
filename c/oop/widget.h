// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#ifndef C_WIDGET_H_
#define C_WIDGET_H_

#include "object.h"

struct ng_widget_s {
#define NG_WIDGET_IMPL \
  NG_OBJECT_IMPL \
  int x; \
  int y; \
  int width; \
  int height;
};
typedef struct ng_widget_s ng_widget_t;
#define NG_WIDGET(widget) (ng_widget_t*)(widget)

extern ng_widget_t* ng_widget_new();

extern void ng_widget_init(ng_widget_t* widget);

extern void ng_widget_destroy(ng_object_t* obj);

extern void ng_wiget_set_size(ng_widget_t* widget, int width, int height);

extern int ng_widget_get_width(ng_widget_t* widget);

#endif  // C_WIDGET_H_
