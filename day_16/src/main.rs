use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use bitmaps::Bitmap;

fn main() {
    let input: HashMap<String, (usize, Vec<String>)> = include_str!("input.txt")
        .lines()
        .map(|line| {
            let splitted_data = line.split("; ").collect::<Vec<&str>>();
            let flow = splitted_data[0].split("=").collect::<Vec<&str>>()[1].parse::<usize>().unwrap();

            let first_char = splitted_data[0].chars().nth(6).unwrap();
            let second_char = splitted_data[0].chars().nth(7).unwrap();
            let identifier: String = vec![first_char, second_char].iter().collect();

            let mut connected_valves = vec![];

            if splitted_data[1].contains("valves") {
                connected_valves = splitted_data[1]
                    .split("tunnels lead to valves ")
                    .collect::<Vec<&str>>()[1]
                    .split(", ")
                    .map(|element| element.to_string())
                    .collect::<Vec<String>>()
            } else {
                connected_valves = splitted_data[1]
                    .split("tunnel leads to valve ")
                    .collect::<Vec<&str>>()[1]
                    .split(", ")
                    .map(|element| element.to_string())
                    .collect::<Vec<String>>()
            }

            (identifier, (flow, connected_valves))
        })
        .collect();

    let name_to_index_matching: HashMap<String, usize> = input
        .iter()
        .enumerate()
        .map(|(index, (identifier, _))| (identifier.clone(), index))
        .collect();

    let mapped_input: HashMap<usize, (usize, Vec<usize>)> = input
        .iter()
        .map(|(identifier, (flow, connected_valves))| {
            (
                name_to_index_matching.get(identifier).unwrap().to_owned(),
                (
                    *flow,
                    connected_valves.iter().map(|valve| *name_to_index_matching.get(valve).unwrap()).collect()
                )
            )
        })
        .collect();

    // Calculate all the distances between the valves using the Floyd–Warshall_algorithm
    let mut distance_matrix: Vec<Vec<usize>> = mapped_input.iter()
        .map(|_| mapped_input.iter().map(|_| usize::MAX).collect::<Vec<usize>>())
        .collect();

    for (index, (_, connected_valves)) in mapped_input.iter() {
        distance_matrix[*index][*index] = 0;

        for other_valve in connected_valves {
            distance_matrix[*index][*other_valve] = 1;
        }
    }

    for k in 0..distance_matrix.len() {
        for i in 0..distance_matrix.len() {
            if distance_matrix[i][k] == usize::MAX {
                continue;
            }

            for j in 0..distance_matrix.len() {
                if distance_matrix[k][j] == usize::MAX {
                    continue;
                }

                if distance_matrix[i][j] > distance_matrix[i][k] + distance_matrix[k][j] {
                    distance_matrix[i][j] = distance_matrix[i][k] + distance_matrix[k][j];
                }
            }
        }
    }

    // Let's reduce the input to pressurized valves only
    let pressurized_valves: HashMap<usize, usize> = mapped_input.into_iter()
        .filter(|(_, (flow, _))| *flow != 0)
        .map(|(index, (flow, _))| (index, flow))
        .collect();

    println!("Solution to part 1: {:?}", part_1(&distance_matrix, &name_to_index_matching, &pressurized_valves));
    println!("Solution to part 2: {:?}", part_2(&distance_matrix, &name_to_index_matching, &pressurized_valves));
}

fn part_1(distance_matrix: &Vec<Vec<usize>>, name_to_index_matching: &HashMap<String, usize>, pressurized_valves: &HashMap<usize, usize>) -> usize {
    // State consists of current_location, opened_valves, total_flow
    let mut dynamic_program: VecDeque<(usize, usize, Bitmap<46>, usize)> = VecDeque::new();

    let start_node = name_to_index_matching.get("AA").unwrap();
    let mut unopened_valves = Bitmap::new();

    for (index, _) in pressurized_valves.iter() {
        unopened_valves.set(*index, true);
    }

    dynamic_program.push_back((30, *start_node, unopened_valves, 0));

    let mut largest_flow = 0;

    while let Some((time_remaining, location, unopened_valves, previous_flow)) = dynamic_program.pop_front() {
        if time_remaining > 0 {
            for unvisited_valve in unopened_valves.clone().into_iter() {
                if unvisited_valve == location {
                    continue;
                }

                // Move to the target valve AND open it
                let distance = distance_matrix[location][unvisited_valve] + 1;

                // If this is not possible, continue and find another valve to reach
                if time_remaining < distance {
                    continue;
                }

                let additional_flow = pressurized_valves.get(&unvisited_valve).unwrap() * (time_remaining - distance);
                largest_flow = max(largest_flow, previous_flow + additional_flow);

                let mut unopened_valves = unopened_valves.clone();
                unopened_valves.set(unvisited_valve, false);

                dynamic_program.push_front((time_remaining - distance, unvisited_valve, unopened_valves, previous_flow + additional_flow));
            }
        }
    }

    largest_flow
}

fn part_2(distance_matrix: &Vec<Vec<usize>>, name_to_index_matching: &HashMap<String, usize>, pressurized_valves: &HashMap<usize, usize>) -> usize {
    let mut dynamic_program: VecDeque<(usize, usize, Bitmap<46>, usize)> = VecDeque::new();

    let start_node = name_to_index_matching.get("AA").unwrap();
    let mut unopened_valves = Bitmap::new();

    for (index, _) in pressurized_valves.iter() {
        unopened_valves.set(*index, true);
    }

    dynamic_program.push_back((26, *start_node, unopened_valves, 0));

    let mut possible_actions: HashMap<Bitmap<46>, usize> = HashMap::new();

    while let Some((time_remaining, location, unopened_valves, previous_flow)) = dynamic_program.pop_front() {
        if time_remaining == 0 {
            let existing_flow = possible_actions.get(&unopened_valves).unwrap_or(&0);
            possible_actions.insert(unopened_valves, max(*existing_flow, previous_flow));
        } else {
            dynamic_program.push_front((0, location, unopened_valves, previous_flow));

            for unvisited_valve in unopened_valves.clone().into_iter() {
                if unvisited_valve == location {
                    continue;
                }

                // Move to the target valve AND open it
                let distance = distance_matrix[location][unvisited_valve] + 1;

                // If this is not possible, continue and find another valve to reach
                if time_remaining < distance {
                    continue;
                }

                let additional_flow =
                    pressurized_valves.get(&unvisited_valve).unwrap() * (time_remaining - distance);

                let mut unopened_valves = unopened_valves.clone();
                unopened_valves.set(unvisited_valve, false);

                dynamic_program.push_front((
                    time_remaining - distance,
                    unvisited_valve,
                    unopened_valves,
                    previous_flow + additional_flow,
                ));
            }
        }
    }

    let mut solution = 0;

    for (me_valves, me_flow) in possible_actions.iter() {
        for (elephant_valves, elephant_flow) in possible_actions.iter() {
            if pressurized_valves.iter().any(|(index, _)| {
                if !elephant_valves.get(*index) && !me_valves.get(*index) {
                    return true;
                }

                false
            }) {
                continue;
            }

            solution = max(solution, me_flow + elephant_flow);
        }
    }

    solution
}