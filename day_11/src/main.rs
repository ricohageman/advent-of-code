use std::collections::VecDeque;

#[derive(Copy, Clone)]
enum Operator {
    Plus(usize),
    Multiply(usize),
    Square,
}

#[derive(Clone)]
struct Monkey {
    pub inventory: VecDeque<usize>,
    pub operation: Operator,
    pub test: usize,
    pub target_true: usize,
    pub target_false: usize
}

fn main() {
    let monkeys = include_str!("input.txt").split("\n\n")
        .map(|string| string.lines().collect::<Vec<_>>())
        .map(|data| {
            let parsed_operator = data[2]
                .split("  Operation: new = old ")
                .collect::<Vec<&str>>()[1]
                .split_whitespace()
                .collect::<Vec<&str>>();

            let operation = match (parsed_operator[0], parsed_operator[1])  {
                ("*", "old") => Operator::Square,
                ("*", number) => Operator::Multiply(number.parse().unwrap()),
                ("+", number) => Operator::Plus(number.parse().unwrap()),
                _ => panic!(),
            };


            Monkey {
                inventory: data[1]
                    .split("Starting items: ")
                    .collect::<Vec<&str>>()[1]
                    .split(", ")
                    .map(|value| value.parse::<usize>().unwrap())
                    .collect::<VecDeque<usize>>(),
                operation,
                test: data[3]
                    .split("  Test: divisible by ")
                    .collect::<Vec<&str>>()[1]
                    .parse::<usize>()
                    .unwrap(),
                target_true: data[4]
                    .split("    If true: throw to monkey ")
                    .collect::<Vec<&str>>()[1]
                    .parse::<usize>()
                    .unwrap(),
                target_false: data[5]
                    .split("    If false: throw to monkey ")
                    .collect::<Vec<&str>>()[1]
                    .parse::<usize>()
                    .unwrap(),
            }
        })
        .collect::<Vec<Monkey>>();

    println!("Solution to part 1: {:?}", part_1(&monkeys));
    println!("Solution to part 2: {:?}", part_2(&monkeys));
}

fn part_1(monkeys: &Vec<Monkey>) -> usize {
    let mut monkey_activity = monkeys.iter().map(|_| 0).collect::<Vec<_>>();
    let mut monkeys = monkeys.clone();

    for _ in 0..20 {
        for monkey_index in 0..monkeys.len() {
            let monkey = &mut monkeys[monkey_index];

            if monkey.inventory.is_empty() {
                continue;
            }

            let mut monkey_inventory = monkey.inventory.clone();
            monkey.inventory.clear();

            let monkey = monkey.clone();

            while let Some(item) = monkey_inventory.pop_front() {
                monkey_activity[monkey_index] += 1;

                let mut new_item_value = match monkey.operation {
                    Operator::Plus(amount) => item + amount,
                    Operator::Multiply(amount) => item * amount,
                    Operator::Square => item * item,
                };

                new_item_value /= 3;

                if new_item_value % monkey.test == 0 {
                    monkeys[monkey.target_true].inventory.push_back(new_item_value);
                } else {
                    monkeys[monkey.target_false].inventory.push_back(new_item_value);
                }
            }
        }
    }

    monkey_activity.sort();
    monkey_activity.reverse();

    monkey_activity[0] * monkey_activity[1]
}

fn part_2(monkeys: &Vec<Monkey>) -> usize {
    let mut monkey_activity = monkeys.iter().map(|_| 0).collect::<Vec<_>>();
    let mut monkeys = monkeys.clone();
    let modulo = monkeys.iter().map(|monkey| monkey.test).fold(1, |acc, value| acc * value);

    for _ in 0..10000 {
        for monkey_index in 0..monkeys.len() {
            let monkey = &mut monkeys[monkey_index];

            if monkey.inventory.is_empty() {
                continue;
            }

            let mut monkey_inventory = monkey.inventory.clone();
            monkey.inventory.clear();

            let monkey = monkey.clone();

            while let Some(item) = monkey_inventory.pop_front() {
                monkey_activity[monkey_index] += 1;

                let mut new_item_value = match monkey.operation {
                    Operator::Plus(amount) => item + amount,
                    Operator::Multiply(amount) => item * amount,
                    Operator::Square => item * item,
                };

                new_item_value = new_item_value % modulo;

                if new_item_value % monkey.test == 0 {
                    monkeys[monkey.target_true].inventory.push_back(new_item_value);
                } else {
                    monkeys[monkey.target_false].inventory.push_back(new_item_value);
                }
            }
        }
    }

    monkey_activity.sort();
    monkey_activity.reverse();

    monkey_activity[0] * monkey_activity[1]
}