// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use searching::symbol_table::SymbolTable;

fn main() {
    let stdin = std::io::stdin();
    let mut buffer = String::new();
    let mut i = 0;

    println!("will read stdin");

    let mut table: SymbolTable<String, usize> = SymbolTable::new();
    loop {
        if let Ok(size) = stdin.read_line(&mut buffer) {
            if size == 0 {
                break;
            }

            table.put(buffer.clone(), i);
            i += 1;
        } else {
            println!("Read to END");
            break;
        }
    }

    println!("size of table: {}", table.size());
    for key in table.keys() {
        println!("key: {}, value: {:?}", key, table.get(&key));
    }
}
