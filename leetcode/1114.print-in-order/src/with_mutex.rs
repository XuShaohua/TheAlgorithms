// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be
// found in the LICENSE file.

#![allow(dead_code)]

use std::sync::{Arc, Mutex};
use std::thread;

struct Foo {
    counter: Mutex<i32>,
}

fn print_first() {
    print!("first");
}

fn print_second() {
    print!("second");
}

fn print_third() {
    print!("third");
}

impl Foo {
    #[must_use]
    pub fn new() -> Self {
        Self {
            counter: Mutex::new(0),
        }
    }

    pub fn first(&self) {
        print_first();
        *self.counter.lock().unwrap() = 1;
    }

    pub fn second(&self) {
        loop {
            let data = self.counter.lock().unwrap();
            if *data == 1 {
                break;
            }
        }
        print_second();
        *self.counter.lock().unwrap() = 2;
    }

    pub fn third(&self) {
        loop {
            let data = self.counter.lock().unwrap();
            if *data == 2 {
                break;
            }
        }
        print_third();
    }
}

pub fn run() {
    let foo = Arc::new(Foo::new());

    let a = {
        let foo = foo.clone();
        thread::spawn(move || {
            foo.first();
        })
    };
    let b = {
        let foo = foo.clone();
        thread::spawn(move || {
            foo.second();
        })
    };
    let c = {
        thread::spawn(move || {
            foo.third();
        })
    };
    let _ = a.join();
    let _ = b.join();
    let _ = c.join();
}
