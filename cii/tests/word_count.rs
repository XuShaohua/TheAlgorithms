use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn word_count(filename: &str) -> Result<(), io::Error> {
    let fd = File::open(filename)?;
    let reader = BufReader::new(fd);
    let mut counts = BTreeMap::<String, usize>::new();

    for line in reader.lines() {
        let line = line?;
        for word in line.split_ascii_whitespace() {
            if let Some(count) = counts.get_mut(word) {
                *count += 1;
            } else {
                counts.insert(word.to_owned(), 1);
            }
        }
    }

    for (key, value) in &counts {
        println!("{key}: {value}");
    }

    Ok(())
}

fn main() {
    for arg in env::args().skip(1) {
        word_count(&arg).unwrap();
    }
}
