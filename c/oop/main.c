// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <assert.h>

#include "button.h"

int main(void) {
  ng_button_t* button = ng_button_new();
  ng_widget_set_size(NG_WIDGET(button), 100, 24);
  assert(ng_widget_get_width(NG_WIDGET(button)) == 100);
  ng_widget_draw(NG_WIDGET(button));
  ng_object_decref(NG_OBJECT(button));

  return 0;
}