// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

struct FizzBuzz {
    n: i32,

    number_mutex: Mutex<i32>,
    number_cvar: Condvar,
}

fn print_num(prefix: &str, x: i32) {
    println!("{prefix}: {x}");
}

impl FizzBuzz {
    #[must_use]
    pub fn new(n: i32) -> Self {
        Self {
            n,
            number_mutex: Mutex::new(1),
            number_cvar: Condvar::new(),
        }
    }

    pub fn fizz(&self) {
        let mut number;
        loop {
            {
                let mut mutex = self.number_mutex.lock().unwrap();
                while *mutex <= self.n && (*mutex % 3 != 0 || *mutex % 5 == 0) {
                    mutex = self.number_cvar.wait(mutex).unwrap();
                }
                if *mutex > self.n {
                    return;
                }
                number = *mutex;
                print_num("fizz", number);
            }

            {
                let mut mutex = self.number_mutex.lock().unwrap();
                *mutex = number + 1;
                self.number_cvar.notify_all();
            }
        }
    }

    pub fn buzz(&self) {
        let mut number;
        loop {
            {
                let mut mutex = self.number_mutex.lock().unwrap();
                while *mutex <= self.n && (*mutex % 5 != 0 || *mutex % 3 == 0) {
                    mutex = self.number_cvar.wait(mutex).unwrap();
                }
                if *mutex > self.n {
                    return;
                }
                number = *mutex;
                print_num("buzz", number);
            }

            {
                let mut mutex = self.number_mutex.lock().unwrap();
                *mutex = number + 1;
                self.number_cvar.notify_all();
            }
        }
    }

    pub fn fizzbuzz(&self) {
        let mut number;
        loop {
            {
                let mut mutex = self.number_mutex.lock().unwrap();
                while *mutex <= self.n && (*mutex % 3 != 0 || *mutex % 5 != 0) {
                    mutex = self.number_cvar.wait(mutex).unwrap();
                }
                if *mutex > self.n {
                    return;
                }
                number = *mutex;
                print_num("fizzbuzz", number);
            }

            {
                let mut mutex = self.number_mutex.lock().unwrap();
                *mutex = number + 1;
                self.number_cvar.notify_all();
            }
        }
    }

    pub fn number(&self) {
        let mut number;
        loop {
            {
                let mut mutex = self.number_mutex.lock().unwrap();
                while *mutex <= self.n && (*mutex % 3 == 0 || *mutex % 5 == 0) {
                    mutex = self.number_cvar.wait(mutex).unwrap();
                }
                if *mutex > self.n {
                    return;
                }
                number = *mutex;
                print_num("number", number);
            }

            {
                let mut mutex = self.number_mutex.lock().unwrap();
                *mutex = number + 1;
                self.number_cvar.notify_all();
            }
        }
    }
}

pub fn run() {
    //let n = 15;
    let n = 30;
    let fizz_buzz = Arc::new(FizzBuzz::new(n));
    let a = {
        let fizz_buzz = Arc::clone(&fizz_buzz);
        thread::spawn(move || {
            fizz_buzz.fizz();
        })
    };
    let b = {
        let fizz_buzz = Arc::clone(&fizz_buzz);
        thread::spawn(move || {
            fizz_buzz.buzz();
        })
    };
    let c = {
        let fizz_buzz = Arc::clone(&fizz_buzz);
        thread::spawn(move || {
            fizz_buzz.fizzbuzz();
        })
    };
    let d = {
        thread::spawn(move || {
            fizz_buzz.number();
        })
    };
    let _ = a.join();
    let _ = b.join();
    let _ = c.join();
    let _ = d.join();
}
