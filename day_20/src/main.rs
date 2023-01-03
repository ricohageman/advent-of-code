fn main() {
    println!("Solution to part 1: {:?}", part_1());
    println!("Solution to part 2: {:?}", part_2());
}

fn part_1() -> i64 {
    let initial_list: Vec<i64> = include_str!("input.txt")
        .lines()
        .map(|input| input.parse::<i64>().unwrap())
        .collect();
    let mut mixed_list: Vec<usize> = (0..initial_list.len()).collect();

    for (initial_index, delta) in initial_list.iter().enumerate() {
        let current_index = mixed_list.iter().position(|i| i == &initial_index).unwrap();
        mixed_list.remove(current_index);

        let updated_index = (current_index as i64 + delta).rem_euclid(mixed_list.len() as i64);
        mixed_list.insert(updated_index as usize, initial_index);
    }

    let starting_position_in_initial_list = initial_list.iter().position(|i| i == &0).unwrap();
    let starting_position = mixed_list.iter().position(|i| i == &starting_position_in_initial_list).unwrap();

    vec![1000, 2000, 3000].iter()
        .map(|amount| initial_list[mixed_list[(amount + starting_position) % mixed_list.len()]])
        .sum::<i64>()
}

fn part_2() -> i64 {
    let initial_list: Vec<i64> = include_str!("input.txt")
        .lines()
        .map(|input| input.parse::<i64>().unwrap() * 811589153)
        .collect();
    let mut mixed_list: Vec<usize> = (0..initial_list.len()).collect();

    for _ in 0..10 {
        for (initial_index, delta) in initial_list.iter().enumerate() {
            let current_index = mixed_list.iter().position(|i| i == &initial_index).unwrap();
            mixed_list.remove(current_index);

            let updated_index = (current_index as i64 + delta).rem_euclid(mixed_list.len() as i64);
            mixed_list.insert(updated_index as usize, initial_index);
        }
    }

    let starting_position_in_initial_list = initial_list.iter().position(|i| i == &0).unwrap();
    let starting_position = mixed_list.iter().position(|i| i == &starting_position_in_initial_list).unwrap();

    vec![1000, 2000, 3000].iter()
        .map(|amount| initial_list[mixed_list[(amount + starting_position) % mixed_list.len()]])
        .sum::<i64>()
}