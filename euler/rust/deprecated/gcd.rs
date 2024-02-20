// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub trait Gcd<Denominator = Self> {
    type Output;
    fn gcd(self, denominator: Denominator) -> Self::Output;
}

macro_rules! gcd_impl {
    ( $( $t:ident )* ) => {
        $(
            impl Gcd for $t {
                type Output = $t;

                /// Implemented based on the Euclidean Algorithm
                fn gcd(self, denominator: $t) -> $t {
                    assert!(self > 0);
                    assert!(denominator > 0);
                    let (mut n, mut d) = if self > denominator {
                        (self, denominator)
                    } else {
                        (denominator, self)
                    };

                    while d != 0 {
                        let r = n % d;
                        n = d;
                        d = r;
                    }

                    n
                }
            }
        )*
    };
}

gcd_impl!(u8 i8 u16 i16 u32 i32 usize isize u64 i64 u128 i128);

pub trait Lcm<Denominator = Self> {
    type Output;
    fn lcm(self, denominator: Denominator) -> Self::Output;
}

macro_rules! lcm_impl {
    ( $( $t:ident )* ) => {
        $(
            impl Lcm for $t {
                type Output = $t;

                /// Based on the theorem: `a * b = gcd(a, b) * lcm(a, b)`
                fn lcm(self, denominator: $t) -> $t {
                    let gcd = self.gcd(denominator);
                    self / gcd * denominator
                }
            }
        )*
    };
}

lcm_impl!(u8 i8 u16 i16 u32 i32 usize isize u64 i64 u128 i128);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(i8::gcd(8, 16), 8);
        assert_eq!(u64::gcd(42, 21), 21);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(i8::lcm(8, 16), 16);
        assert_eq!(u64::lcm(42, 21), 42);
        assert_eq!(u64::lcm(42, 21), 42);
    }
}
