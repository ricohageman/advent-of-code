use std::collections::HashSet;

fn split_coordinate(string: &str) -> (isize, isize) {
    let split: Vec<&str> = string.split(", ").collect();

    let x = split[0].split("=").collect::<Vec<&str>>()[1].parse().unwrap();
    let y = split[1].split("=").collect::<Vec<&str>>()[1].parse().unwrap();

    (x, y)
}

fn part_1(data: &Vec<((isize, isize), (isize, isize))>, distance_per_sensor_beacon_pair: &Vec<isize>) -> usize {
    // Part 1: Find all the positions in row y that cannot contain a beacon.
    let target_y = 2000000;
    let mut invalid_positions: HashSet::<isize> = HashSet::new();

    for (index, ((x, y), (_, _))) in data.iter().enumerate() {
        let distance_straight_line = (y - target_y).abs();
        let distance_of_sensor = distance_per_sensor_beacon_pair[index];

        if distance_straight_line > distance_of_sensor {
            continue;
        }

        invalid_positions.insert(*x);

        let remaining_distance = (distance_of_sensor - distance_straight_line).abs();

        for additional_x in 0..=remaining_distance {
            invalid_positions.insert(*x-additional_x);
            invalid_positions.insert(*x+additional_x);
        }
    }

    for (_, (beacon_x, beacon_y)) in data {
        if *beacon_y == target_y {
            invalid_positions.remove(&beacon_x);
        }
    }

    invalid_positions.len()
}

fn move_from_to(
    start: (isize, isize),
    end: (isize, isize),
    range: (isize, isize),
    sensors: &[&(isize, isize)],
    distances: &[isize],
) -> Option<isize> {
    let dx = (end.0 - start.0).signum();
    let dy = (end.1 - start.1).signum();

    let (mut x, mut y) = start.clone();

    while (x, y) != end {
        if x >= range.0 && x <= range.1 && y >= range.0 && y <= range.1 {
            if !sensors.iter().enumerate().any(|(index, (sx, sy))| sx.abs_diff(x) + sy.abs_diff(y) <= distances[index] as usize) {
                return Some(x * 4000000 + y);
            }
        }

        x += dx;
        y += dy;
    }

    None
}

fn part_2(data: &Vec<((isize, isize), (isize, isize))>, distance_per_sensor_beacon_pair: &Vec<isize>) -> isize {
    let range = (0, 4000000);
    let sensors = data.iter().map(|(coordinate, _)| coordinate).collect::<Vec<_>>();

    for (index, (x, y)) in sensors.iter().enumerate() {
        let distance = distance_per_sensor_beacon_pair[index] + 1;

        // Start at the bottom of the diamond and move to the right
        if let Some(result) = move_from_to((*x, y - distance), (x + distance, *y), range, &sensors, &distance_per_sensor_beacon_pair) {
            return result;
        }

        // Start at the right and move to the top
        if let Some(result) = move_from_to((x + distance, *y), (*x, y + distance), range, &sensors, &distance_per_sensor_beacon_pair) {
            return result;
        }

        // Start at the top and move to the left
        if let Some(result) = move_from_to((*x, y + distance), (x - distance, *y), range, &sensors, &distance_per_sensor_beacon_pair) {
            return result;
        }

        // Start at the left and move to the bottom
        if let Some(result) = move_from_to((x - distance, *y), (*x, y - distance), range, &sensors, &distance_per_sensor_beacon_pair) {
            return result;
        }
    }

    1
}

fn main() {
    let data: Vec<((isize, isize), (isize, isize))> =  include_str!("input.txt")
        .lines()
        .map(|line| line.split(":"))
        .map(|mut split| (split_coordinate(split.next().unwrap()), split_coordinate(split.next().unwrap())))
        .collect();

    let distance_per_sensor_beacon_pair: Vec<isize> = data
        .iter()
        .map(|((sx, sy), (bx, by))| (sx - bx).abs() + (sy - by).abs())
        .collect();

    println!("Solution to part 1: {:?}", part_1(&data, &distance_per_sensor_beacon_pair));
    println!("Solution to part 2: {:?}", part_2(&data, &distance_per_sensor_beacon_pair));
}
