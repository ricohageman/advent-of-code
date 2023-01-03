fn part_1(stacks: &Vec<Vec<String>>, operations: &Vec<(usize, usize, usize)>) {
    let mut stacks = stacks.clone();

    for (number_of_movements, from_stack, to_stack) in operations {
        for _ in 0..*number_of_movements {
            let element = stacks[*from_stack].pop().unwrap();
            stacks[*to_stack].push(element);
        }
    }

    println!("Solution to part 1: {:?}", stacks.iter_mut().map(|stack| stack.pop().unwrap()).collect::<Vec<_>>());
}

fn part_2(stacks: &Vec<Vec<String>>, operations: &Vec<(usize, usize, usize)>) {
    let mut stacks = stacks.clone();

    for (number_of_movements, from_stack, to_stack) in operations {
        let mut intermediate_stack = vec![];

        for _ in 0..*number_of_movements {
            let element = stacks[*from_stack].pop().unwrap();
            intermediate_stack.push(element);
        }

        intermediate_stack.reverse();

        for element in intermediate_stack {
            stacks[*to_stack].push(element);
        }
    }

    println!("Solution to part 2: {:?}", stacks.iter_mut().map(|stack| stack.pop().unwrap()).collect::<Vec<_>>());
}

fn main() {
    let stacks = include_str!("start.txt")
            .lines()
            .map(|n| {
                n.parse::<String>()
                    .unwrap()
                    .split(",")
                    .map(|element| element.parse::<String>().unwrap())
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();

    let operations = include_str!("operations.txt")
        .lines()
        .map(|n| {
            let parsed_string = n.parse::<String>().unwrap();
            let all_numbers = parsed_string
                .split_whitespace()
                .filter_map(|word| word.parse::<usize>().ok())
                .collect::<Vec<usize>>();

            assert_eq!(all_numbers.len(), 3);
            assert!(all_numbers[1] - 1 < stacks.len());
            assert!(all_numbers[2] - 1 < stacks.len());

            (all_numbers[0], all_numbers[1] - 1, all_numbers[2] - 1)
        })
        .collect::<Vec<(usize, usize, usize)>>();

    part_1(&stacks, &operations);
    part_2(&stacks, &operations);
}
