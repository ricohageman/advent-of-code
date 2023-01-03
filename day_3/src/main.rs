use std::collections::HashSet;

fn main() {
    let alphabet = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    println!(
        "Solution to part 1: {:?}",
        include_str!("input.txt")
            .lines()
            .map(|n| {
                let parsed_string = n.parse::<String>().unwrap();

                let priorities = parsed_string.chars()
                    .map(|element| alphabet.find(element).unwrap())
                    .collect::<Vec<usize>>();

                let rucksack_size = parsed_string.len() / 2;

                let mut first_compartment: HashSet<usize> = HashSet::default();
                let mut second_compartment: HashSet<usize> = HashSet::default();

                for (index, priority) in priorities.into_iter().enumerate() {
                    match index >= rucksack_size {
                        true => second_compartment.insert(priority),
                        false => first_compartment.insert(priority),
                    };
                }

                first_compartment.intersection(&second_compartment).sum::<usize>()
            })
            .sum::<usize>()
    );

    println!(
        "Solution to part 2: {:?}",
        include_str!("input.txt")
            .lines()
            .map(|n| {
                let parsed_string = n.parse::<String>().unwrap();

                parsed_string.chars()
                    .map(|element| alphabet.find(element).unwrap())
                    .collect::<HashSet<usize>>()
            })
            .collect::<Vec<HashSet<usize>>>()
            .chunks(3)
            .map(|elves| {
                elves.into_iter()
                    .fold(None, |acc: Option<HashSet<usize>>, elve| {
                        acc.map(|acc| acc.intersection(elve).cloned().collect())
                            .or(Some(elve.clone()))
                    })
                    .unwrap()
                    .iter()
                    .sum::<usize>()
            })
            .sum::<usize>()
    );
}
