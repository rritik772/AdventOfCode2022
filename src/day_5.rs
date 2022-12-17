pub fn run(lines: Vec<String>) -> String {
    let mut crates = vec![vec![]];
    let mapping: [usize; 9] = [1, 5, 9, 13, 17, 21, 25, 29, 33];

    for i in mapping {
        let mut temp = vec![];

        for line in (0..8).rev() {
            let c = lines[line].chars().nth(i).unwrap();

            if c != ' ' {
                temp.push(c)
            }
        }

        crates.push(temp);
    }

    crates.remove(0);

    let mut instructions: Vec<[usize; 3]> = vec![];

    for i in 10..lines.len() {
        let str_list: Vec<&str> = lines[i].split(' ').collect();

        instructions.push([
            str_list[1].parse::<usize>().unwrap(),
            str_list[3].parse::<usize>().unwrap(),
            str_list[5].parse::<usize>().unwrap(),
        ])
    }

    second(&mut crates, instructions)
}

fn first(crates: &mut Vec<Vec<char>>, instructions: Vec<[usize; 3]>) -> String {
    let mut result = "".to_owned();

    for instruction in instructions {
        let moves = instruction[0];
        let from = instruction[1] - 1;
        let to = instruction[2] - 1;

        for _ in 0..moves {
            let tmp = crates[from].pop().unwrap();
            crates[to].push(tmp);
        }
    }

    for _crate in crates {
        result.push(_crate.pop().unwrap());
    }

    result
}

fn second(crates: &mut Vec<Vec<char>>, instructions: Vec<[usize; 3]>) -> String {
    let mut result = "".to_owned();

    for instruction in instructions {

        let moves = instruction[0];
        let from = instruction[1] - 1;
        let to = instruction[2] - 1;

        let mut tmp = vec![];

        for _ in 0..moves {
            tmp.insert(0, crates[from].pop().unwrap());
        }

        // println!("tmp: {:?}", tmp);
        // println!("before: {:?}", crates[to]);
        crates[to].append(&mut tmp);
        // println!("after: {:?}", crates[to]);
    }

    for _crate in crates {
        result.push(_crate.pop().unwrap_or(' '));
    }


    result
}
