#![allow(dead_code)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

fn main() {
    let file = File::open("./src/input").expect("File not found");
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = vec![];

    for line in reader.lines() {
        let line = line.expect("No line was found.");

        lines.push(line);
    }

    let answer = day_9::run(lines);
    println!("{:?}", answer);
}
