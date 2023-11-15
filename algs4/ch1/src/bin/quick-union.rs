// Copyright (c) 2020 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use ch1::connectivity::{Connectivity, UnionFile};
use ch1::error::Error;
use ch1::quick_union::QuickUnion;

fn main() -> Result<(), Error> {
    let mut uf = UnionFile::new_from_stdin()?;
    let length = uf.read_length()?;
    let mut qu = QuickUnion::new(length);

    loop {
        match uf.read_union() {
            Ok(Some((a, b))) => {
                qu.union(a, b);
            }
            Ok(None) => break,
            Err(err) => {
                eprintln!("{:?}", err);
                break;
            }
        }
    }
    qu.generate_graph()?;

    Ok(())
}
