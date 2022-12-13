use std::{char, iter};

pub fn run(lines: Vec<Vec<char>>) -> u32 {
    second(lines)
}

pub fn first(lines: Vec<Vec<char>>) -> u32 {
    let mut total = 0;

    let mut first_list; // = [0; 2*26];
    let mut second_list; // = [0; 2*26];

    for line in lines {
        first_list = [0; 2 * 26];
        second_list = [0; 2 * 26];

        let length = line.len();
        let first = &line[..length / 2];
        let second = &line[length / 2..];

        for word in first {
            if word.is_uppercase() {
                first_list[*word as usize - 65 + 26] += 1;
            } else if word.is_lowercase() {
                first_list[*word as usize - 97] += 1;
            }
        }

        for word in second {
            if word.is_uppercase() {
                second_list[*word as usize - 65 + 26] += 1;
            } else if word.is_lowercase() {
                second_list[*word as usize - 97] += 1;
            }
        }

        for (i, (x, y)) in iter::zip(&first_list, &second_list).enumerate() {
            if x >= &1 && y >= &1 {
                total += i + 1;
            }
        }
    }

    total as u32
}

pub fn second(lines: Vec<Vec<char>>) -> u32 {
    let mut total = 0;

    for three_elfs in lines.chunks(3) {
        let mut char_counts = vec![[0; 2*26]; 3];

        for (i, elf) in three_elfs.iter().enumerate() {
            for c in elf {
                if c.is_lowercase() {
                    char_counts[i][*c as u32 as usize - 97] += 1;
                } else {
                    char_counts[i][*c as u32 as usize - 65 + 26] += 1;
                }
            }
        }

        for i in 0..52 {
            if i < 26 {
                println!("i: {}, char: {}, 1: {}, 2: {}, 3: {}", i, (i + 97) as u8 as char, char_counts[0][i], char_counts[1][i], char_counts[2][i] )
            } else {
                println!("i: {}, char: {}, 1: {}, 2: {}, 3: {}", i, (i + 65 - 26) as u8 as char, char_counts[0][i], char_counts[1][i], char_counts[2][i] )
            }

            if char_counts[0][i] > 0 && char_counts[1][i] > 0 && char_counts[2][i] > 0 {
                if i < 26 {
                    total += i + 1;
                } else {
                    total += i + 1;
                }
            }
        }
    }

    total as u32
}
