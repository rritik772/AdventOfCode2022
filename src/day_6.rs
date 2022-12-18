use std::collections::HashMap;

pub fn run(lines: Vec<String>) -> Vec<usize> {
    let mut result = vec![];
    for line in lines {
        let ans = second(&line);
        result.push(ans);
    }
    result
}

fn first(s: &String) -> usize {
    let chars = s.chars().collect::<Vec<char>>();
    let len = s.len() - 4;

    let mut stage = 0;

    for i in 0..len {
        let a = chars[i];
        let b = chars[i + 1];
        let c = chars[i + 2];
        let d = chars[i + 3];

        if a != b && a != c && a != d && b != c && b != d && c != d{
            stage = i + 3 + 1;
            break;
        }
    }

    return stage
}

fn second(s: &String) -> usize {
    let chars = s.chars().collect::<Vec<char>>();
    let len = s.len() - 14;

    let stage = 0;

    for i in 0..len {
        let chunk = &chars[i..i+14];
        let mut count = HashMap::new();

        for c in chunk {
            count.entry(c).or_insert(0);
            count.insert(c, count.get(c).unwrap() + 1);
        }

        println!("{:?}", count);

        if count.len() == 14 {
            return i + 5;
        }

    }

    return stage
}

fn count_char(mapping: &[u32; 26]) -> u32 {
    let mut count = 0;

    for i in mapping {
        if *i > 0 {
            count += 1
        }
    }

    return count;
}
