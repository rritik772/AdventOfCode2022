#![allow(dead_code)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod day_1;
mod day_2;
mod day_3;

fn main() {
    let file = File::open("./src/input").expect("File not found");
    let reader = BufReader::new(file);

    let mut lines: Vec<Vec<char>> = vec![];

    for line in reader.lines() {
        let line = line.expect("No line was found.");

        let converted = line
            .split(" ")
            .flat_map(|s| s.chars())
            .collect::<Vec<char>>();

        lines.push(converted);
    }

    let answer = day_3::run(lines);
    println!("{}", answer);
}
