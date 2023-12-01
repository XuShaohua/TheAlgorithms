// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include "widget.h"

#include <assert.h>
#include <stdlib.h>
#include <stdio.h>

static void ng_widget_draw_noop(ng_widget_t* widget) {
  assert(widget != NULL);
  fprintf(stderr, "%s, do nothing\n", __func__);
}

ng_widget_t* ng_widget_new() {
  ng_widget_t* widget = (ng_widget_t*)malloc(sizeof(ng_widget_t));
  assert(widget != NULL);
  ng_widget_init(widget);
  return widget;
}

void ng_widget_init(ng_widget_t* widget) {
  assert(widget != NULL);
  ng_object_init(NG_OBJECT(widget));
  widget->destroy = ng_widget_destroy;
  widget->draw = ng_widget_draw_noop;
  widget->x = 0;
  widget->y = 0;
  widget->width = 100;
  widget->height = 100;
}

void ng_widget_destroy(ng_object_t* obj) {
  fprintf(stderr, "%s: %p\n", __func__, obj);
  assert(obj != NULL);
  ng_widget_t* widget = (ng_widget_t*)obj;
  ng_object_destroy(obj);
}

void ng_widget_set_size(ng_widget_t* widget, int width, int height) {
  assert(widget != NULL);
  widget->width = width;
  widget->height = height;
}

int ng_widget_get_width(ng_widget_t* widget) {
  assert(widget != NULL);
  return widget->width;
}

void ng_widget_draw(ng_widget_t* widget) {
  fprintf(stderr, "%s, %p\n", __func__, widget);
  assert(widget != NULL);
  widget->draw(widget);
}

