// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#![allow(dead_code)]

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct ZeroEvenOdd {
    n: i32,

    zero_mutex: Mutex<bool>,
    zero_cvar: Condvar,

    even_mutex: Mutex<i32>,
    even_cvar: Condvar,

    odd_mutex: Mutex<i32>,
    odd_cvar: Condvar,
}

fn print_number(_prefix: &str, x: i32) {
    print!("{x}");
}

impl ZeroEvenOdd {
    pub fn new(n: i32) -> Self {
        Self {
            n,

            zero_mutex: Mutex::new(true),
            zero_cvar: Condvar::new(),

            even_mutex: Mutex::new(0),
            even_cvar: Condvar::new(),

            odd_mutex: Mutex::new(0),
            odd_cvar: Condvar::new(),
        }
    }

    pub fn zero(&self) {
        for i in 0..=self.n {
            {
                let mut zero_mutex = self.zero_mutex.lock().unwrap();
                while !*zero_mutex {
                    zero_mutex = self.zero_cvar.wait(zero_mutex).unwrap();
                }
                *zero_mutex = false;
            }
            if i == self.n {
                break;
            }

            print_number("zero", 0);

            let next_num = i + 1;
            if next_num % 2 == 0 {
                let mut num_mutex = self.even_mutex.lock().unwrap();
                *num_mutex = next_num;
                self.even_cvar.notify_one();
            } else {
                let mut num_mutex = self.odd_mutex.lock().unwrap();
                *num_mutex = next_num;
                self.odd_cvar.notify_one();
            }
        }

        {
            let mut num_mutex = self.even_mutex.lock().unwrap();
            *num_mutex = -1;
            self.even_cvar.notify_one();
        }
        {
            let mut num_mutex = self.odd_mutex.lock().unwrap();
            *num_mutex = -1;
            self.odd_cvar.notify_one();
        }
    }

    pub fn even(&self) {
        loop {
            let even_num;
            {
                let mut num_mutex = self.even_mutex.lock().unwrap();
                loop {
                    if *num_mutex == -1 {
                        return;
                    }
                    if *num_mutex != 0 {
                        break;
                    }
                    num_mutex = self.even_cvar.wait(num_mutex).unwrap();
                }

                even_num = *num_mutex;
                *num_mutex = 0;
            }

            print_number("even", even_num);
            {
                let mut zero_mutex = self.zero_mutex.lock().unwrap();
                *zero_mutex = true;
                self.zero_cvar.notify_one();
            }
        }
    }

    pub fn odd(&self) {
        loop {
            let odd_num;
            {
                let mut num_mutex = self.odd_mutex.lock().unwrap();
                loop {
                    if *num_mutex == -1 {
                        return;
                    }
                    if *num_mutex != 0 {
                        break;
                    }
                    num_mutex = self.odd_cvar.wait(num_mutex).unwrap();
                }

                odd_num = *num_mutex;
                *num_mutex = 0;
            }

            print_number("odd", odd_num);

            {
                let mut zero_mutex = self.zero_mutex.lock().unwrap();
                *zero_mutex = true;
                self.zero_cvar.notify_one();
            }
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
