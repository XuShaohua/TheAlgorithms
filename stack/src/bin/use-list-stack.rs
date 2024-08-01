// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use stack::list_stack::ListStack;

fn main() {
    let mut stack: ListStack<i32> = ListStack::new();
    assert!(stack.is_empty());
    stack.push(10);
    stack.push(20);
    stack.push(30);
    stack.push(40);
    stack.push(50);
    assert_eq!(stack.len(), 5);

    let ret = stack.pop();
    assert_eq!(ret, Some(50));
    let ret = stack.pop();
    assert_eq!(ret, Some(40));
    assert!(!stack.is_empty());
    let ret = stack.pop();
    assert_eq!(ret, Some(30));
    let ret = stack.pop();
    assert_eq!(ret, Some(20));
    let ret = stack.pop();
    assert_eq!(ret, Some(10));
    assert!(stack.is_empty());

    let ret = stack.pop();
    assert_eq!(ret, None);
}