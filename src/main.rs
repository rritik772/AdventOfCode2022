#![allow(dead_code)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
    let file = File::open("./src/input").expect("File not found");
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = vec![];

    for line in reader.lines() {
        let line = line.expect("No line was found.");

        lines.push(line);
    }

    let answer = day_4::run(lines);
    println!("{}", answer);
}
