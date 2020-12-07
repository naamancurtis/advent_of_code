#![allow(dead_code)]

extern crate aoc_runner;
#[macro_use]
extern crate aoc_runner_derive;

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;

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

aoc_lib! { year = 2020 }
