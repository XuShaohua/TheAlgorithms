// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub trait CountDigits {
    fn count_digits(&self) -> u16;
}

macro_rules! count_digits_impl {
    ( $( $t:ident )* ) => {
        $(
            impl CountDigits for $t {
                fn count_digits(&self) -> u16 {
                    let mut count = 1;
                    let mut n = *self;
                    while n >= 10 {
                        count += 1;
                        n /= 10;
                    }
                    count
                }
            }
        )*
    };
}

count_digits_impl!(u8 u16 u32 u64 u128);

pub trait GetDigits {
    fn get_digits(&self, buf: &mut Vec<u8>) -> usize;
}

macro_rules! get_digits_impl {
    ( $( $t:ident )* ) => {
        $(
            impl GetDigits for $t {
                fn get_digits(&self, buf: &mut Vec<u8>) -> usize {
                    let mut n = *self;
                    let old_len = buf.len();
                    while n > 0 {
                        buf.push((n % 10) as u8);
                        n /= 10;
                    }
                    buf.len() - old_len
                }
            }
        )*
    };
}

get_digits_impl!(u8 u16 u32 u64 u128);
