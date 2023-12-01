// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#include <assert.h>
#include <stddef.h>

struct ng_object_s {
  size_t ref_count;
  size_t (*hash)(struct ng_object_s* obj);
  void (destroy*)(struct ng_object_s* obj);
};
typedef struct ng_object_s ng_object_t;

size_t hash_object_address(ng_object_t* obj) {
  return (unsigned long)obj >> 1;
}

ng_object_t* ng_object_new() {
  ng_object_t* obj = (ng_object_t*)malloc(sizeof(ng_object_t));
  assert(obj != NULL);
  obj->ref_count = 1;
  obj->hash = hash_object_address;
  obj->destroy = ng_object_destroy;
  return obj;
}

void ng_object_destroy(ng_object_t* obj) {
  assert(obj != NULL);
  free(obj);
}

void ng_object_incref(ng_object_t* obj) {
  assert(obj != NULL);
  obj->ref_count += 1;
}

void ng_object_decref(ng_object_t* obj) {
  assert(obj != NULL);
  assert(obj->ref_count > 0);
  obj->ref_count -= 1;
  if (obj->ref_count == 0) {
    obj->destroy(obj);
  }
}

struct ng_widget_s {
  ng_object_t* obj;
  int width;
  int height;
  int x;
  int y;
};
typedef struct ng_widget_s ng_widget_t;

ng_widget_t* ng_widget_new() {
  ng_widget_t* widget = (ng_widget_t*)malloc(sizeof(ng_widget_t));
  assert(widget);
  widget->obj = ng_object_new();
  widget->obj->destroy = ng_widget_destroy;
  widget->x = 0;
  widget->y = 0;
  widget->width = 100;
  widget->height = 100;
  return widget;
}

void ng_widget_destroy(ng_object_t* obj) {
  assert(obj != NULL);
  ng_widget_t* widget = (ng_widget_t*)obj;
}

void ng_wiget_set_size(ng_widget_t* widget, int width, int height) {
  assert(widget != NULL);
  widget->width = width;
  widget->height = height;
}

int ng_widget_get_width(ng_widget_t* widget) {
  assert(widget != NULL);
  return widget->width;
}

struct ng_button_s {
  ng_widget_t* widget;
  const char* label;
};

typedef struct ng_button_s ng_button_t;

ng_button_t* ng_button_new() {
  ng_button_t* button = (ng_button_t*)malloc(sizeof(ng_button_t));
  button->widget = ng_widget_new();
  assert(button != NULL);

  return button;
}

#define NG_WIDGET(widget) widget->widget

int main(void) {
  ng_button_t* button = ng_button_new();
  ng_widget_set_size(button, 100, 24);
  assert(ng_widget_get_width(NG_WIDGET(button)), 100);

  return 0;
}
