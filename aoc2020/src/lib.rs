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
pub mod day_8;
pub mod day_9;
pub mod day_10;
pub mod day_11;
pub mod day_12;

use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

pub fn get_input_from_file<P: AsRef<Path>>(path: P) -> String {
    let file = File::open(path).expect("file failed to open");
    let mut buf = String::new();
    BufReader::new(file).read_to_string(&mut buf).ok();
    buf
}

aoc_lib! { year = 2020 }
