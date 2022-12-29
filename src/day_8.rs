pub fn run(lines: Vec<String>) -> usize {
    second(lines)
}

fn first(lines: Vec<String>) -> usize {
    let mut total = 0;
    let mut visibility = vec![vec![false; lines[0].len()]; lines.len()];

    // left
    for (i, line) in lines.iter().enumerate() {
        let mut gt = usize::MIN;

        for (j, c) in  line.chars().enumerate() {
            let curr = c as usize - '0' as usize;

            if curr > gt {
                gt = curr;
                visibility[i][j] = true;
            }
        }

    }

    // top
    for i in 1..lines[0].len() - 1 {
        let mut gt = usize::MIN;

        for j in 0..lines.len() {
            let c = lines[j].chars().nth(i).unwrap() as usize - '0' as usize;

            if c > gt {
                gt = c;
                visibility[j][i] = true;
            }
        }
    }

    // right
    for (i, line) in lines.iter().enumerate() {
        let mut gt = usize::MIN;

        for (j, e) in line.chars().rev().enumerate() {
            let c = e as usize - '0' as usize;

            if c > gt {
                gt = c;
                visibility[i][lines[0].len() - j - 1] = true;
            }
        }
    }

    // down
    for i in 1..lines[0].len() {
        let mut gt = usize::MIN;

        for j in (0..lines.len()).rev() {
            let c = lines[j].chars().nth(i).unwrap() as usize - '0' as usize;

            if c > gt {
                visibility[j][i] = true;
                gt = c;
            }
        }
    }

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if i == 0 || i == lines.len() - 1 ||
                j == 0 || j == lines[i].len() - 1 {
                    visibility[i][j] = true;
                }
        }
    }

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if visibility[i][j] == true { total += 1; }
        }
    }

    println!("Final");
    print_visibility(&visibility);
    total
}

fn print_visibility(visibility: &Vec<Vec<bool>>) {
    for line in visibility {
        println!("{:?}", line);
    }
}

fn second(lines: Vec<String>) -> usize {
    let mut result = vec![vec![0; lines[0].len()]; lines.len()];
    
    for (i, line) in lines.iter().enumerate() {
        if i == 0 || i == lines.len() - 1 { continue; }

        for (j, e) in line.chars().enumerate() {
            if j == 0 || j == lines[i].len() - 1 { continue; }

            let curr = e as usize - '0' as usize;
            let last = line.chars().nth(j - 1).unwrap() as usize - '0' as usize;

            if last < curr {
                result[i][j] = 1 + result[i][j - 1];
            } else {
                result[i][j] = 1;
            }
        }
    }

    println!("left");
    for line in result.clone() {
        println!("{:?}", line);
    }

    for i in 1..lines[0].len()-1 {
        for j in 1..lines.len()-1 {
            let curr = lines[j].chars().nth(i).unwrap() as usize - '0' as usize;
            let last = lines[j].chars().nth(i - 1).unwrap() as usize - '0' as usize;

            if last < curr {
                result[j][i] *= 1 + result[i][j - 1];
            } else {
                result[j][i] *= 1;
            }
        }
    }

    println!("top");
    for line in result.clone() {
        println!("{:?}", line);
    }

    for i in 1..lines.len()-1 {
        for j in (1..lines[i].len()-1).rev() {
            let curr = lines[i].chars().nth(j).unwrap() as usize - '0' as usize;
            let last = lines[i].chars().nth(j + 1).unwrap() as usize - '0' as usize;

            if last < curr {
                result[i][j] *= 1 + result[i][j + 1];
            } else {
                result[i][j] *= 1;
            }
        }
    }

    println!("right");
    for line in result.clone() {
        println!("{:?}", line);
    }

    for i in 1..lines[0].len() - 1 {
        for j in (1..lines.len() - 1).rev() {
            let curr = lines[i].chars().nth(j).unwrap() as usize - '0' as usize;
            let last = lines[i].chars().nth(j + 1).unwrap() as usize - '0' as usize;

            if last < curr {
                result[i][j] *= 1 + result[i][j + 1];
            } else {
                result[i][j] *= 1;
            }
        }
    }

    println!("down");
    for line in result.clone() {
        println!("{:?}", line);
    }


    let mut max = usize::MIN;
    for (_, line) in result.iter().enumerate() {
        for (_, e) in line.iter().enumerate() {
            if *e > max {
                max = *e as usize;
            }
        }
    }

    max
}
