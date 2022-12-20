use std::{collections::HashMap, path::PathBuf, str::FromStr};

pub fn run(lines: Vec<String>) -> u64 {
    second(lines)
}

fn first(lines: Vec<String>) -> usize {
    let mut sizes = HashMap::new();
    let mut dirs = Vec::new();

    for line in lines {

        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let words: Vec<_> = line.split_whitespace().collect();

        match words[..] {
            ["$", "cd", ".."] => { dirs.pop(); },

            ["$", "cd", name] => { dirs.push(name.to_owned()); },

            [size, _name] => {
                let size: usize = size.parse().unwrap();
                
                for i in 0..dirs.len() {
                    let path = PathBuf::from_iter(&dirs[..=i]);

                    *sizes.entry(path).or_insert(0) += size;
                }
            },

            _ => {}
        }

    }

    sizes
        .into_values()
        .filter(|x| *x < 100_000)
        .sum()
}

const DISKSIZE: u64 = 7_00_00_000;
const NEED: u64 = 3_00_00_000;

fn second(lines: Vec<String>) -> u64 {
    let mut sizes = HashMap::new();
    let mut dirs = Vec::new();

    for line in lines {

        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let words: Vec<_> = line.split_whitespace().collect();

        match words[..] {
            ["$", "cd", ".."] => { dirs.pop(); },

            ["$", "cd", name] => { dirs.push(name.to_owned()); },

            [size, _name] => {
                let size: u64 = size.parse().unwrap();
                
                for i in 0..dirs.len() {
                    let path = PathBuf::from_iter(&dirs[..=i]);

                    *sizes.entry(path).or_insert(0) += size;
                }
            },

            _ => {}
        }

    }

    let root = sizes.get(&PathBuf::from("/")).unwrap();
    let available = DISKSIZE - root;

    sizes
        .into_values()
        .filter(|x| available + x >= NEED )
        .min()
        .unwrap()
}
