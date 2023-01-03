fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let grid = include_str!("input.txt")
        .lines()
        .map(|n| n.chars().map(|char| char.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();

    let mut visible_grid: Vec<Vec<bool>> = grid
        .iter()
        .map(|row| {
            row.iter()
                .map(|_| false)
                .collect()
        })
        .collect();

    for x in 0..grid.len() {
        // Go from left to right
        let mut current_maximum = None;

        for y in 0..grid.len() {
            let tree = grid[x][y];
            if current_maximum.is_none() || tree > current_maximum.unwrap() {
                current_maximum = Some(tree);
                visible_grid[x][y] = true;
            }
        }

        // Go from right to left
        let mut current_maximum = None;

        for y in (0..grid.len()).rev() {
            let tree = grid[x][y];
            if current_maximum.is_none() || tree > current_maximum.unwrap() {
                current_maximum = Some(tree);
                visible_grid[x][y] = true;
            }
        }

        // Go from top to bottom
        let mut current_maximum = None;

        for y in 0..grid.len() {
            let tree = grid[y][x];
            if current_maximum.is_none() || tree > current_maximum.unwrap() {
                current_maximum = Some(tree);
                visible_grid[y][x] = true;
            }
        }

        // Go from bottom to top
        let mut current_maximum = None;

        for y in (0..grid.len()).rev() {
            let tree = grid[y][x];
            if current_maximum.is_none() || tree > current_maximum.unwrap() {
                current_maximum = Some(tree);
                visible_grid[y][x] = true;
            }
        }
    }

    println!("Solution to part 1: {:?}", visible_grid.iter().map(|row| row.iter().filter(|&&x| x).count()).sum::<usize>())
}

fn part_2() {
    let grid = include_str!("input.txt")
        .lines()
        .map(|n| n.chars().map(|char| char.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();

    let mut highest_scenic_score: Option<usize> = None;

    for x in 0..grid.len() {
        for y in 0..grid.len() {
            let height = grid[x][y];
            let mut scenic_score = 1;

            // Look left
            let mut current_view = 0;
            for index in (0..y).rev() {
                current_view += 1;

                if grid[x][index] >= height {
                    break;
                }
            }

            scenic_score *= current_view;

            // Look right
            let mut current_view = 0;
            for index in (y+1)..grid.len() {
                current_view += 1;

                if grid[x][index] >= height {
                    break;
                }
            }

            scenic_score *= current_view;

            // Look left
            let mut current_view = 0;
            for index in (x+1)..grid.len() {
                current_view += 1;

                if grid[index][y] >= height {
                    break;
                }
            }

            scenic_score *= current_view;

            // Look left
            let mut current_view = 0;
            for index in (0..x).rev() {
                current_view += 1;

                if grid[index][y] >= height {
                    break;
                }
            }

            scenic_score *= current_view;

            if highest_scenic_score.is_none() || highest_scenic_score.unwrap() < scenic_score {
                highest_scenic_score = Some(scenic_score);
            }
        }
    }

    println!("Solution to part 2: {:?}", highest_scenic_score.unwrap());
}
