// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

pub fn exch<T>(list: &mut Vec<T>, i: usize, j: usize)
where
    T: Clone,
{
    let tmp = list[i].clone();
    list[i] = list[j].clone();
    list[j] = tmp.clone();
}

pub fn is_sorted<T>(list: &[T]) -> bool
where
    T: PartialOrd,
{
    for i in 0..(list.len() - 1) {
        if list[i] > list[i + 1] {
            return false;
        }
    }
    return true;
}

pub fn show<T>(vec: &[T])
where
    T: std::fmt::Display,
{
    for s in vec {
        print!("{} ", s);
    }
    println!();
}

#[allow(dead_code)]
struct Generics<T> {
    t: std::marker::PhantomData<T>,
}

impl<T: Copy + std::fmt::Display> Generics<T> {
    #[allow(dead_code)]
    pub fn exch(list: &mut Vec<T>, i: usize, j: usize) {
        let tmp = list[i];
        list[i] = list[j];
        list[j] = tmp;
    }

    #[allow(dead_code)]
    pub fn show(vec: &Vec<T>) {
        for s in vec {
            print!("{} ", s);
        }
        println!("");
    }
}
