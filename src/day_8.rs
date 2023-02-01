pub fn run() -> usize {
    main()
}

fn first(lines: Vec<String>) -> usize {
    let mut total = 0;
    let mut visibility = vec![vec![false; lines[0].len()]; lines.len()];

    // left
    for (i, line) in lines.iter().enumerate() {
        let mut gt = usize::MIN;

        for (j, c) in line.chars().enumerate() {
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
            if i == 0 || i == lines.len() - 1 || j == 0 || j == lines[i].len() - 1 {
                visibility[i][j] = true;
            }
        }
    }

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if visibility[i][j] == true {
                total += 1;
            }
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
        if i == 0 || i == lines.len() - 1 {
            continue;
        }

        for (j, e) in line.chars().enumerate() {
            if j == 0 || j == lines[i].len() - 1 {
                continue;
            }

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

    for i in 1..lines[0].len() - 1 {
        for j in 1..lines.len() - 1 {
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

    for i in 1..lines.len() - 1 {
        for j in (1..lines[i].len() - 1).rev() {
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct GridCoord {
    x: usize,
    y: usize,
}

impl std::fmt::Debug for GridCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![T::default(); width * height],
        }
    }

    fn in_bounds(&self, coord: GridCoord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    fn cell_mut(&mut self, coord: GridCoord) -> Option<&mut T> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&mut self.data[coord.y * self.width + coord.x])
    }

    fn cell(&self, coord: GridCoord) -> Option<&T> {
        if !self.in_bounds(coord) {
            return None;
        }
        Some(&self.data[coord.y * self.width + coord.x])
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }
}

fn parse_grid(input: &str) -> Grid<usize> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut grid = Grid::new(width, height);
    for (y, line) in input.lines().enumerate() {
        for (x, col) in line.chars().enumerate() {
            assert!(col.is_ascii_digit());
            *grid.cell_mut((x, y).into()).unwrap() = col as usize - '0' as usize;
        }
    }

    grid
}

fn main() -> usize {
    let grid = parse_grid(include_str!("./input"));
    // println!("grid at (0, 0): {:?}", grid.cell((0, 0).into()));
    // println!("grid at (3, 0): {:?}", grid.cell((3, 0).into()));
    // println!("grid at (0, 2): {:?}", grid.cell((0, 2).into()));

    part_1(grid)
}

fn part_1(grid: Grid<usize>) -> usize {
    let all_coords = (0..grid.height()).into_iter().flat_map(|y| {
        (0..grid.width())
            .into_iter()
            .map(move |x| GridCoord::from((x, y)))
    });

    let nums_visible_cells = all_coords
        .filter(|&coord| {
            let coord_height = grid.cell(coord).unwrap();
            let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];

            deltas.iter().any(|&(dx, dy)| {
                let mut cells_in_line = (1..).into_iter().map_while(|i| {
                    let coord = GridCoord {
                        x: coord.x.checked_add_signed(dx * i)?,
                        y: coord.y.checked_add_signed(dy * i)?,
                    };

                    grid.cell(coord)
                });

                cells_in_line.all(|height| height < coord_height)
            })
        })
        .count();

    return nums_visible_cells;
}
