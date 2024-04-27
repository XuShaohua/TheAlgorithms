// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#![allow(dead_code)]

use std::hint;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;

struct ZeroEvenOdd {
    n: i32,
    counter: AtomicI32,
}

fn print_number(x: i32) {
    print!("{x}");
}

impl ZeroEvenOdd {
    const ZERO: i32 = 0;
    const ODD: i32 = 1;
    const EVEN: i32 = 2;

    pub fn new(n: i32) -> Self {
        Self {
            n,
            counter: AtomicI32::new(Self::ZERO),
        }
    }

    pub fn zero(&self) {
        for i in 0..self.n {
            while self.counter.load(Ordering::SeqCst) != Self::ZERO {
                hint::spin_loop();
            }

            print_number(0);
            let is_even = if (i + 1) % 2 == 0 {
                Self::EVEN
            } else {
                Self::ODD
            };
            self.counter.store(is_even, Ordering::SeqCst);
        }
    }

    pub fn even(&self) {
        for i in (2..=self.n).step_by(2) {
            while self.counter.load(Ordering::SeqCst) != Self::EVEN {
                hint::spin_loop();
            }

            print_number(i);
            self.counter.store(Self::ZERO, Ordering::SeqCst);
        }
    }

    pub fn odd(&self) {
        for i in (1..=self.n).step_by(2) {
            while self.counter.load(Ordering::SeqCst) != Self::ODD {
                hint::spin_loop();
            }

            print_number(i);
            self.counter.store(Self::ZERO, Ordering::SeqCst);
        }
    }
}

pub fn run() {
    //let n = 2;
    let n = 5;
    let zero_even_odd = Arc::new(ZeroEvenOdd::new(n));
    let a = {
        let zero_even_odd = Arc::clone(&zero_even_odd);
        thread::spawn(move || {
            zero_even_odd.zero();
        })
    };
    let b = {
        let zero_even_odd = Arc::clone(&zero_even_odd);
        thread::spawn(move || {
            zero_even_odd.even();
        })
    };
    let c = thread::spawn(move || {
        zero_even_odd.odd();
    });
    let _ = a.join();
    let _ = b.join();
    let _ = c.join();
}
