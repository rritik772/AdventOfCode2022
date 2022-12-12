#![allow(dead_code)]

pub fn run(lines: Vec<Vec<u32>>) -> u32 {
    return second(lines);
}

fn first(lines: Vec<Vec<u32>>) -> u32 {
    let mut max = u32::MIN;

    for elf in lines {
        let sum = elf.iter().fold(0, |acc, x| acc + x);

        if sum > max {
            max = sum;
        }
    }

    return max;
}

fn second(lines: Vec<Vec<u32>>) -> u32 {
    let mut h1 = u32::MIN;
    let mut h2 = u32::MIN;
    let mut h3 = u32::MIN;

    for elf in lines {
        let sum: u32 = elf.iter().sum();

        if sum > h1 {
            h3 = h2;
            h2 = h1;
            h1 = sum;
        } else if sum > h2 {
            h3 = h2;
            h2 = sum;
        } else if sum > h3 {
            h3 = sum;
        }
    }

    return h1 + h2 + h3;
}
