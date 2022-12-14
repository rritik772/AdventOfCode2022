pub fn run(lines: Vec<String>) -> u32 {
    let lines = sanatise(lines);
    second(lines)
}

fn sanatise(lines: Vec<String>) -> Vec<Vec<Vec<u32>>> {
    let mut result = Vec::new();

    for line in lines {
        let elfs = line.split(',').collect::<Vec<&str>>();
        let mut ranges = vec![];

        for elf in elfs {
            let single_elf_ranges = elf
                .split('-')
                .map(|s| s.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            
            ranges.push(single_elf_ranges);
        }

        result.push(ranges);
    }

    result
}

fn first(lines: Vec<Vec<Vec<u32>>>) -> u32 {
    let mut total = 0;

    for line in lines {
        let s1 = line[0][0];
        let e1 = line[0][1];

        let s2 = line[1][0];
        let e2 = line[1][1];


        if s1 >= s2 && e1 <= e2 || s1 <= s2 && e1 >= e2 {
            total += 1;
        }
    }

    total
}

fn second(lines: Vec<Vec<Vec<u32>>>) -> u32 {
    let mut total = 0;

    for line in lines {
        let s1 = line[0][0];
        let e1 = line[0][1];

        let s2 = line[1][0];
        let e2 = line[1][1];

        if !(e1 < s2 || e2 < s1) {
            total += 1;
        }
    }

    total
}
