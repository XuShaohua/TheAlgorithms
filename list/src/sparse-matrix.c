// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

struct coordinate_s {
  int row;
  int column;
};

struct sparse_matrix_s {
  struct coordinate_s coord;
  int value;
  struct sparse_matrix_s* next_row;
  struct sparse_matrix_s* next_column;
};