use std::collections::HashMap;

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub enum Rock {
    Horizontal,
    Cross,
    Corner,
    Vertical,
    Square,
}

impl Rock {
    fn solid_points(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        match self {
            Rock::Horizontal => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            Rock::Cross => vec![
                (x, y + 1),
                (x + 1, y + 1),
                (x + 2, y + 1),
                (x + 1, y),
                (x + 1, y + 2),
            ],
            Rock::Corner => vec![
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 2, y + 1),
                (x + 2, y + 2),
            ],
            Rock::Vertical => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            Rock::Square => vec![(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)],
        }
    }
}

struct TallChamber {
    grid: Vec<bool>,
    height: usize,
}

impl TallChamber {
    fn new() -> Self {
        Self {
            grid: vec![false; 7],
            height: 1,
        }
    }

    fn coord_to_index(&self, x: usize, y: usize) -> usize {
        y * 7 + x
    }

    pub fn is_occupied(&self, x: usize, y: usize) -> bool {
        *self.grid.get(self.coord_to_index(x, y)).unwrap_or(&false)
    }

    pub fn occupy_points(&mut self, points: &[(usize, usize)]) {
        for (x, y) in points {
            let index = self.coord_to_index(*x, *y);

            while index >= self.grid.len() {
                self.grid.extend(vec![false; 7]);
                self.height += 1;
            }

            self.grid[index] = true;
        }
    }

    pub fn normalized_skyline(&self) -> [usize; 7] {
        let skyline = self.skyline();
        let smallest_height = skyline.iter().min().unwrap();

        [
            skyline[0] - smallest_height,
            skyline[1] - smallest_height,
            skyline[2] - smallest_height,
            skyline[3] - smallest_height,
            skyline[4] - smallest_height,
            skyline[5] - smallest_height,
            skyline[6] - smallest_height,
        ]
    }

    fn skyline(&self) -> [usize; 7] {
        [
            self.column_height(0),
            self.column_height(1),
            self.column_height(2),
            self.column_height(3),
            self.column_height(4),
            self.column_height(5),
            self.column_height(6),
        ]
    }

    fn column_height(&self, x: usize) -> usize {
        (0..self.height)
            .filter(|height| self.is_occupied(x, *height))
            .last()
            .unwrap_or(0)
    }
}


fn main() {
    println!("Solution to part 1: {:?}", part_1());
    println!("Solution to part 2: {:?}", part_2());
}

fn part_1() -> usize {
    let rock_pattern = vec![
        Rock::Horizontal,
        Rock::Cross,
        Rock::Corner,
        Rock::Vertical,
        Rock::Square,
    ];

    let mut rock_pattern_iterator = rock_pattern.iter().cycle();
    let mut operation_pattern = include_str!("input.txt").chars().cycle();

    let mut current_rock = rock_pattern_iterator.next().unwrap();
    let (mut x, mut y) = (2, 3);

    let mut number_of_rocks = 0;

    let mut chamber = TallChamber::new();

    while number_of_rocks < 2022 {
        // After a rock appears, it alternates between being pushed by a jet of hot gas ...
        x = match operation_pattern.next() {
            Some('>') => {
                if !current_rock
                    .solid_points(x, y)
                    .iter()
                    .any(|(x, y)| *x == 6 || chamber.is_occupied(*x + 1, *y))
                {
                    x + 1
                } else {
                    x
                }
            }
            Some('<') => {
                if !current_rock
                    .solid_points(x, y)
                    .iter()
                    .any(|(x, y)| *x == 0 || chamber.is_occupied(*x - 1, *y))
                {
                    x - 1
                } else {
                    x
                }
            }
            _ => panic!("No new operation found"),
        };

        // ... and then falling one unit down.
        if current_rock
            .solid_points(x, y)
            .iter()
            .any(|(x, y)| *y == 0 || chamber.is_occupied(*x, *y - 1)) {
            chamber.occupy_points(&current_rock.solid_points(x, y));

            // If a downward movement would have caused a falling rock to move into the floor or an already-fallen rock,
            // the falling rock stops where it is (having landed on something) and a new rock immediately begins falling.
            current_rock = rock_pattern_iterator.next().unwrap();

            // Each rock appears so that its left edge is two units away from the left wall
            // and its bottom edge is three units above the highest rock in the room (or the floor, if there isn't one).
            x = 2;
            y = 3 + chamber.height;
            number_of_rocks += 1;
        } else {
            y -= 1;
        }
    }

    chamber.height
}

fn part_2() -> usize {
    let rock_pattern = vec![
        Rock::Horizontal,
        Rock::Cross,
        Rock::Corner,
        Rock::Vertical,
        Rock::Square,
    ];

    let mut rock_pattern_iterator = rock_pattern.iter().cycle();
    let mut operation_pattern = include_str!("input.txt").chars().enumerate().cycle();
    let required_number_of_rocks = 1000000000000;

    let mut current_rock = rock_pattern_iterator.next().unwrap();
    let (mut x, mut y) = (2, 3);

    let mut number_of_rocks = 0;
    let mut additional_height = 0;

    let mut chamber = TallChamber::new();
    let mut history: HashMap<Option<(Rock, usize)>, (usize, usize)> = HashMap::new();
    let mut skipped = false;

    while number_of_rocks < required_number_of_rocks {
        // After a rock appears, it alternates between being pushed by a jet of hot gas ...
        let (operation_index, operation) = operation_pattern.next().unwrap();

        if !skipped && chamber.normalized_skyline() == [0; 7] {
            let current_key = Some((*current_rock, operation_index));

            if let Some((previous_number_of_rocks, previous_height)) = history.get(&current_key) {
                let cycle_length = number_of_rocks - previous_number_of_rocks;
                let rocks_remaining = required_number_of_rocks - number_of_rocks;
                let cycles = ((rocks_remaining as f64) / (cycle_length as f64)).floor() as usize;
                let rocks_skipped = cycles * cycle_length;

                let delta_height_per_cycle = chamber.height - previous_height;
                let total_delta_height = delta_height_per_cycle * cycles;

                // Let's take some short-cut
                println!(
                    "Identified a loop of '{:?}' rocks and a height difference of '{:?}' skipping '{:?}' cycles to start at '{:?}' instead of '{:?}'",
                    cycle_length,
                    delta_height_per_cycle,
                    cycles,
                    number_of_rocks + rocks_skipped,
                    number_of_rocks
                );

                number_of_rocks += rocks_skipped;
                additional_height += total_delta_height;
                skipped = true;
            } else {
                history.insert(current_key, (number_of_rocks, chamber.height));
            }
        }

        x = match operation {
            '>' => {
                if !current_rock
                    .solid_points(x, y)
                    .iter()
                    .any(|(x, y)| *x == 6 || chamber.is_occupied(*x + 1, *y))
                {
                    x + 1
                } else {
                    x
                }
            }
            '<' => {
                if !current_rock
                    .solid_points(x, y)
                    .iter()
                    .any(|(x, y)| *x == 0 || chamber.is_occupied(*x - 1, *y))
                {
                    x - 1
                } else {
                    x
                }
            }
            _ => panic!("No new operation found"),
        };

        // ... and then falling one unit down.
        if current_rock
            .solid_points(x, y)
            .iter()
            .any(|(x, y)| *y == 0 || chamber.is_occupied(*x, *y - 1)) {
            chamber.occupy_points(&current_rock.solid_points(x, y));

            // If a downward movement would have caused a falling rock to move into the floor or an already-fallen rock,
            // the falling rock stops where it is (having landed on something) and a new rock immediately begins falling.
            current_rock = rock_pattern_iterator.next().unwrap();

            // Each rock appears so that its left edge is two units away from the left wall
            // and its bottom edge is three units above the highest rock in the room (or the floor, if there isn't one).
            x = 2;
            y = 3 + chamber.height;
            number_of_rocks += 1;
        } else {
            y -= 1;
        }
    }

    chamber.height + additional_height
}