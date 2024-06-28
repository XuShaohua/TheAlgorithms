// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(_c: &mut Criterion) {}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
