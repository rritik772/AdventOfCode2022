use std::collections::{HashMap, HashSet};

pub fn run(lines: Vec<String>) -> usize {
    let lines = lines
        .iter()
        .map(|x| {
            let s: Vec<_> = x.split(' ').map(|a| a.parse::<String>().unwrap()).collect();

            let c = s[0].chars().nth(0).unwrap();
            let i = usize::from_str_radix(&s[1], 10).unwrap();
            (c, i)
        })
        .collect::<Vec<_>>();

    first(lines)
}

fn first(lines: Vec<(char, usize)>) -> usize {
    let mut tail_pos: HashSet<(_, _)> = HashSet::from([(0, 0)]);
    let mut h = vec![0, 0];
    let mut t = vec![0, 0];

    let moves: HashMap<char, (usize, i32)> =
        HashMap::from([('L', (0, -1)), ('U', (1, 1)), ('R', (0, 1)), ('D', (0, -1))]);

    for (d, num) in lines {
        let (axis, sign) = moves.get(&d).unwrap();

        h[*axis] += sign * num as i32;

        if (h[*axis] - t[*axis]).abs() > 1 {
            // check diagonals
            if h[1 - *axis] != t[1 - *axis] {
                t[1 - *axis] = h[1 - *axis];
                t[*axis] += sign;
                tail_pos.insert((t[0], t[1]));
            }

            while (h[*axis] - t[*axis]).abs() > 1 {
                t[*axis] += sign;
                tail_pos.insert((t[0], t[1]));
            }
        }
    }

    tail_pos.len()
}
