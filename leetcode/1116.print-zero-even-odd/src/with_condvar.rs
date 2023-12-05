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

fn print_number(_prefix: &str, x: i32) {
    print!("{x}");
}

impl ZeroEvenOdd {
    pub fn new(n: i32) -> Self {
        Self {
            n: 2 * n,

            mutex: Mutex::new(1),
            cvar: Condvar::new(),
        }
    }

    pub fn zero(&self) {
        loop {
            let mut mutex = self.mutex.lock().unwrap();
            while *mutex <= self.n && *mutex % 2 == 0 {
                mutex = self.cvar.wait(mutex).unwrap();
            }
            if *mutex > self.n {
                return;
            }
            print_number("zero", 0);
            *mutex += 1;
            self.cvar.notify_all();
        }
    }

    pub fn even(&self) {
        loop {
            let mut mutex = self.mutex.lock().unwrap();
            while *mutex <= self.n && (*mutex % 2 != 0 || *mutex % 4 != 0) {
                mutex = self.cvar.wait(mutex).unwrap();
            }
            if *mutex > self.n {
                return;
            }
            print_number("even", *mutex / 2);
            *mutex += 1;
            self.cvar.notify_all();
        }
    }

    pub fn odd(&self) {
        loop {
            let mut mutex = self.mutex.lock().unwrap();
            while *mutex <= self.n && (*mutex % 2 != 0 || *mutex % 4 == 0) {
                mutex = self.cvar.wait(mutex).unwrap();
            }
            if *mutex > self.n {
                return;
            }
            print_number("odd", *mutex / 2);
            *mutex += 1;
            self.cvar.notify_all();
        }
    }
}

pub fn run() {
    let n = 2;
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
