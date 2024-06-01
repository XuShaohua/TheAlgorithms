// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use diesel::connection::SimpleConnection;
use std::env;
use std::fs;

mod db;
mod error;

use error::Error;

fn main() -> Result<(), Error> {
    dotenvy::dotenv()?;
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = db::get_connection_pool()?;
    let mut conn = pool.get()?;
    for arg in env::args().skip(1) {
        let sql_content: String = fs::read_to_string(arg)?;
        conn.batch_execute(&sql_content)?;
    }
    Ok(())
}
