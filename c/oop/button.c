// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include "button.h"

#include <assert.h>
#include <stdio.h>
#include <stdlib.h>

ng_button_t* ng_button_new() {
  ng_button_t* button = (ng_button_t*)malloc(sizeof(ng_button_t));
  assert(button != NULL);
  ng_button_init(button);
  return button;
}

void ng_button_init(ng_button_t* button) {
  assert(button != NULL);
  ng_widget_init(NG_WIDGET(button));
  button->destroy = ng_button_destroy;
  button->draw = ng_button_draw;
  button->label = "";
}

void ng_button_destroy(ng_object_t* obj) {
  fprintf(stderr, "%s: %p\n", __func__, obj);
  assert(obj != NULL);
  ng_widget_destroy(NG_OBJECT(obj));
}

void ng_button_draw(ng_widget_t* widget) {
  assert(widget != NULL);
  fprintf(stderr, "%s draw buttons at %dx%d, size: %dx%d\n", __func__,
      widget->x, widget->y, widget->width, widget->height);
}
