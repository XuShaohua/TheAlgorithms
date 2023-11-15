// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::error::{Error, ErrorKind};

pub trait Connectivity {
    fn len(&self) -> usize;
    fn union(&mut self, a: usize, b: usize);
    fn is_connected(&self, a: usize, b: usize) -> bool;
    fn generate_graph(&mut self) -> Result<String, Error>;
}

#[derive(Debug)]
pub struct UnionFile {
    reader: BufReader<File>,
}

impl UnionFile {
    pub fn new<P: AsRef<Path>>(filename: P) -> Result<Self, Error> {
        let file = File::open(filename.as_ref())?;
        let reader = BufReader::new(file);
        Ok(Self { reader })
    }

    pub fn new_from_stdin() -> Result<Self, Error> {
        let mut args = std::env::args();
        let filename = args.nth(1).ok_or(Error::new(
            ErrorKind::ParamError,
            "No union file specified in param!",
        ))?;
        let file = File::open(&filename)?;
        let reader = BufReader::new(file);
        Ok(Self { reader })
    }

    pub fn read_length(&mut self) -> Result<usize, Error> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        let cap: usize = line.trim_end().parse().map_err(|err| {
            Error::from_string(
                ErrorKind::ParseIntError,
                format!("Failed to parse uf length for line: {}, err: {}", line, err),
            )
        })?;
        Ok(cap)
    }

    pub fn read_union(&mut self) -> Result<Option<(usize, usize)>, Error> {
        let mut line = String::new();
        let size = self.reader.read_line(&mut line)?;
        if size == 0 {
            return Ok(None);
        }
        let mut split = line.split(' ');
        let a_str = split.next().ok_or(Error::from_string(
            ErrorKind::ParseIntError,
            format!("Failed to parse union record from: {}", line),
        ))?;
        let a = a_str.parse().map_err(|err| {
            Error::from_string(
                ErrorKind::ParseIntError,
                format!("Failed to parse union.a for line: {}, err: {}", line, err),
            )
        })?;
        let b_str = split.next().ok_or(Error::from_string(
            ErrorKind::ParseIntError,
            format!("Failed to parse union record from: {}", line),
        ))?;
        let b = b_str.trim_end().parse().map_err(|err| {
            Error::from_string(
                ErrorKind::ParseIntError,
                format!("Failed to parse union.b for line: {}, err: {}", line, err),
            )
        })?;
        Ok(Some((a, b)))
    }
}
