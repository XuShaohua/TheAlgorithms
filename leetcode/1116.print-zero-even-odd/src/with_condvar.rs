// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#![allow(dead_code)]

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct ZeroEvenOdd {
    n: i32,
    mutex: Mutex<i32>,
    cvar: Condvar,
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
            mutex: Mutex::new(Self::ODD),
            cvar: Condvar::new(),
        }
    }

    pub fn zero(&self) {
        for i in 0..self.n {
            eprintln!("zero() {i}");
            let mut counter = self.mutex.lock().unwrap();
            while *counter != Self::ZERO {
                eprintln!("zero counter: {}", *counter);
                counter = self.cvar.wait(counter).unwrap();
            }
            print_number(0);
            *counter = if i % 2 == 0 { Self::EVEN } else { Self::ODD };
            self.cvar.notify_all();
        }
    }

    pub fn even(&self) {
        for i in (0..self.n).step_by(2) {
            eprintln!("even() {i}");
            let mut counter = self.mutex.lock().unwrap();
            while *counter != Self::EVEN {
                eprintln!("even counter: {}", *counter);
                counter = self.cvar.wait(counter).unwrap();
            }
            print_number(i);
            *counter = Self::ZERO;
            self.cvar.notify_all();
        }
    }

    pub fn odd(&self) {
        for i in (1..self.n).step_by(2) {
            eprintln!("odd() {i}");
            let mut counter = self.mutex.lock().unwrap();
            while *counter != Self::ODD {
                eprintln!("odd counter: {}", *counter);
                counter = self.cvar.wait(counter).unwrap();
            }

            print_number(i);
            *counter = Self::ZERO;
            self.cvar.notify_all();
        }
    }
}

pub fn run() {
    let n = 1;
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
