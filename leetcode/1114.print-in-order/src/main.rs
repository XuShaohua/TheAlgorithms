// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

mod with_atomic;
mod with_mutex;

fn main() {
    with_mutex::run();
    //with_atomic::run();
}
