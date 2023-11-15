// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use std::fmt::Display;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use crate::error::{Error, ErrorKind};

#[derive(Debug)]
pub struct Graph {
    nodes: Vec<String>,
}

impl Graph {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_node<T: Display + PartialEq>(&mut self, a: T, b: T) -> &mut Self {
        if a == b {
            self.nodes.push(format!("{} [color=red, style=filled]", a));
        }
        self.nodes.push(format!("{} -> {}", a, b));
        self
    }

    pub fn output_str(&self) -> Result<String, Error> {
        let mut s = String::new();
        writeln!(s, "digraph {{")?;
        writeln!(s, "  rankdir = \"BT\"")?;
        writeln!(s, "  ordering = \"in\"")?;
        for node in &self.nodes {
            writeln!(s, "  {}", node)?;
        }
        writeln!(s, "}}")?;
        Ok(s)
    }

    /// Output svg image.
    pub fn quick_output(&self, filename: &str) -> Result<(), Error> {
        let dot_file = format!("{}.dot", filename);
        self.output_dot(&dot_file)?;
        let image_file = format!("{}.svg", filename);
        self.output_image(&dot_file, &image_file)?;
        Ok(())
    }

    pub fn output_dot<P: AsRef<Path>>(&self, filepath: P) -> Result<(), Error> {
        let s = self.output_str()?;
        let mut file = File::create(filepath.as_ref())?;
        file.write_all(s.as_bytes())?;
        Ok(())
    }

    pub fn output_image(&self, dot_file: &str, svg_file: &str) -> Result<(), Error> {
        Command::new("dot")
            .args(["-Tsvg", "-o", svg_file, dot_file])
            .spawn()
            .map_err(|err| {
                Error::from_string(
                    ErrorKind::DotError,
                    format!("Failed to convert dot file {}, reason: {:?}", dot_file, err),
                )
            })?;
        Ok(())
    }
}
