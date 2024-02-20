// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub trait ConcateNumber {
    fn concate(&self, other: Self) -> Self;
}

macro_rules! concate_number_impl {
    ( $( $t:ident )* ) => {
        $(
            impl ConcateNumber for $t {
                fn concate(&self, other: Self) -> Self {
                    let mut shift = 1;
                    let mut q = other;
                    while q >= 10 {
                        q /= 10;
                        shift += 1;
                    }
                    self * (10 as $t).pow(shift) + other
                }
            }
        )*
    };
}

concate_number_impl!(u8 u16 u32 usize u64 u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concate_number() {
        assert_eq!(12_u32.concate(42), 1242);
        assert_eq!(42_u64.concate(10), 4210);
    }
}
