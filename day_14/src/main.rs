fn string_to_coord(element: &str) -> (isize, isize) {
    let mut data = element.split(",");
    let x = data.next().unwrap().parse::<isize>().unwrap();
    let y = data.next().unwrap().parse::<isize>().unwrap();

    (x, y)
}

fn construct_grid(file: &'static str) -> Vec<Vec<bool>> {
    let mut grid: Vec<Vec<bool>> = vec![vec![false; 1000]; 1000];

    // Step 1. Construct a grid
    for line in file.lines() {
        let mut elements = line.split(" -> ");
        let (mut current_x, mut current_y) = string_to_coord(elements.next().unwrap());

        while let Some(element) = elements.next() {
            let (x, y) = string_to_coord(element);

            let dx = x - current_x;
            let dy = y - current_y;

            while current_x != x || current_y != y {
                grid[current_x as usize][current_y as usize] = true;
                current_x += dx.signum();
                current_y += dy.signum();
            }

            grid[current_x as usize][current_y as usize] = true;
        }
    }

    grid
}

fn determine_abyss(grid: &Vec<Vec<bool>>) -> usize {
    grid.iter()
        .enumerate()
        .filter_map(|(_, row)| row.iter().enumerate().filter(|(_, x)| **x).map(|(index, _)| index).max())
        .max()
        .unwrap()
}

fn main() {
    // Part 1: How many parts of sand fit the structure before they end up in the abyss?
    let mut grid = construct_grid(include_str!("input.txt"));
    let abyss = determine_abyss(&grid);

    let mut amount_of_sand_allocated = 0;
    loop {
        let (mut x, mut y) = (500, 0);

        loop {
            if y > abyss {
                break;
            }

            if !grid[x][y+1] {
                y += 1;
            } else if !grid[x-1][y+1] {
                y += 1;
                x -= 1;
            } else if !grid[x+1][y+1] {
                y += 1;
                x += 1;
            } else {
                break;
            }
        }

        if y > abyss {
            break;
        }

        amount_of_sand_allocated += 1;
        grid[x as usize][y as usize] = true;
    }

    println!("Solution to part 1: {}", amount_of_sand_allocated);

    // Part 2: How many parts of sand fit the structure before they fill up (500, 0) provided that abyss + 2 is
    //  an infinite horizontal line.
    let mut grid = construct_grid(include_str!("input.txt"));
    let abyss = determine_abyss(&grid) + 2;

    let mut amount_of_sand_allocated = 0;
    loop {
        let (mut x, mut y) = (500, 0);

        loop {
            grid[x-1][abyss] = true;
            grid[x][abyss] = true;
            grid[x+1][abyss] = true;

            if !grid[x][y+1] {
                y += 1;
            } else if !grid[x-1][y+1] {
                y += 1;
                x -= 1;
            } else if !grid[x+1][y+1] {
                y += 1;
                x += 1;
            } else {
                break;
            }
        }

        amount_of_sand_allocated += 1;

        if (x, y) == (500, 0) {
            break;
        }

        grid[x as usize][y as usize] = true;
    }

    println!("Solution to part 2: {}", amount_of_sand_allocated);
}