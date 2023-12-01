// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include "button.h"

#include <assert.h>

ng_button_t* ng_button_new() {
  ng_button_t* button = (ng_button_t*)malloc(sizeof(ng_button_t));
  assert(button != NULL);
  ng_button_init(button);
  return button;
}

void ng_button_init(ng_button_t* button) {
  assert(button != NULL);
  button->label = "";
  ng_widget_init(NG_WIDGET(button));
}

void ng_button_destroy(ng_object_t* obj) {
  fprintf(stderr, "%s: %p\n", __func__, obj);
  assert(obj != NULL);
  ng_widget_destroy(NG_WIDGET(obj));
}
