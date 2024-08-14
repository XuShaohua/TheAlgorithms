// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub trait IsZero: Copy {
    fn is_zero(self) -> bool;
    fn is_not_zero(self) -> bool {
        !self.is_zero()
    }
}

macro_rules! impl_is_zero {
    ($($t:ty)*) => {$(
    impl IsZero for $t {
        fn is_zero(self) -> bool {
            self == 0
        }
    }
    )*}
}

impl_is_zero! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }

impl IsZero for f32 {
    fn is_zero(self) -> bool {
        self == 0.0
    }
}

impl IsZero for f64 {
    fn is_zero(self) -> bool {
        self == 0.0
    }
}