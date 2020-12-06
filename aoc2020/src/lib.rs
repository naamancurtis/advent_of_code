#![allow(dead_code)]

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub fn get_input_from_file<P: AsRef<Path>>(path: P) -> Vec<String> {
    let file = File::open(path).expect("file failed to open");
    let reader = BufReader::new(file);
    reader.lines().filter_map(|s| s.ok()).collect()
}
