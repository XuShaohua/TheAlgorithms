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

fn print_number(x: i32) {
    println!("{x}");
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
            eprintln!("zero() {i}");
            let mut zero_mutex = self.zero_mutex.lock().unwrap();
            while !*zero_mutex {
                zero_mutex = self.zero_cvar.wait(zero_mutex).unwrap();
            }

            println!("print_number(0);");
            print_number(0);

            let next_num = i + 1;
            println!("next_num: {next_num}");
            if next_num % 2 == 0 {
                let mut mutex = self.even_mutex.lock().unwrap();
                *mutex = next_num;
                self.even_cvar.notify_one();
            } else {
                let mut mutex = self.odd_mutex.lock().unwrap();
                *mutex = next_num;
                self.odd_cvar.notify_one();
            }
        }
        {
            let mut mutex = self.even_mutex.lock().unwrap();
            *mutex = 0;
            self.even_cvar.notify_one();
        }
        {
            let mut mutex = self.odd_mutex.lock().unwrap();
            *mutex = 0;
            self.odd_cvar.notify_one();
        }
    }

    pub fn even(&self) {
        loop {
            let mut even_mutex = self.even_mutex.lock().unwrap();
            while *even_mutex == 0 || *even_mutex % 2 == 1 {
                eprintln!("even counter: {}", *even_mutex);
                even_mutex = self.even_cvar.wait(even_mutex).unwrap();
            }

            if *even_mutex == -1 {
                break;
            }
            print_number(*even_mutex);
            let mut zero_mutex = self.zero_mutex.lock().unwrap();
            *zero_mutex = true;
            self.zero_cvar.notify_all();
        }
    }

    pub fn odd(&self) {
        loop {
            let mut odd_mutex = self.odd_mutex.lock().unwrap();
            while *odd_mutex == 0 || *odd_mutex % 2 == 0 {
                eprintln!("odd counter: {}", *odd_mutex);
                odd_mutex = self.odd_cvar.wait(odd_mutex).unwrap();
            }
            if *odd_mutex == -1 {
                break;
            }
            print_number(*odd_mutex);

            let mut zero_mutex = self.zero_mutex.lock().unwrap();
            *zero_mutex = true;
            self.zero_cvar.notify_all();
        }
    }
}

pub fn run() {
    let n = 2;
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
