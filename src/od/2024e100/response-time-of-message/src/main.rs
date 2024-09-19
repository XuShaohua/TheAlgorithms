// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::io::{stdin, BufRead};

fn get_resp_time(req_time: i32) -> i32 {
    if req_time < 128 {
        req_time
    } else {
        let mant: i32 = req_time & 0b1111;
        let exp: i32 = (req_time >> 4) & 0b0110;
        (mant | 0x10) << (exp + 3)
    }
}

fn main() {
    let mut line = String::new();
    let ret = stdin().lock().read_line(&mut line);
    assert!(ret.is_ok());
    let num_request: usize = line.trim().parse().unwrap();

    // 读取所有的请求报文
    let mut req_list = Vec::with_capacity(num_request);
    for line in stdin().lock().lines() {
        let line = line.unwrap();
        let mut parts = line.split_ascii_whitespace();
        let delay: i32 = parts.next().unwrap().parse().unwrap();
        let req_time: i32 = parts.next().unwrap().parse().unwrap();
        req_list.push((delay, req_time));
    }
    assert_eq!(num_request, req_list.len());

    // 计算每个请求报文的响应时间, 并找到最小的值
    let mut min_resp_time = i32::MAX;
    for (delay, req_time) in req_list {
        let resp_time = get_resp_time(req_time);
        let abs_resp_time = delay + resp_time;
        min_resp_time = min_resp_time.min(abs_resp_time);
    }

    println!("{min_resp_time}");
}
