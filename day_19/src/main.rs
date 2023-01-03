use std::cmp::max;
use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
pub struct Blueprint {
    pub id: usize,
    pub ore_robot: usize,
    pub clay_robot: usize,
    pub obsidian_robot: (usize, usize),
    pub geode_robot: (usize, usize),
}

impl Blueprint {
    fn new(id: usize, ore_robot: usize, clay_robot: usize, obsidian_robot: (usize, usize), geode_robot: (usize, usize)) -> Self {
        Self {
            id,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot
        }
    }
}

fn main() {
    let blueprints: Vec<Blueprint> = include_str!("input.txt")
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let line = line.split(". ").collect::<Vec<&str>>();
            let ore_robot = line[0].split("Each ore robot costs ")
                .collect::<Vec<&str>>()[1]
                .split(" ore")
                .collect::<Vec<&str>>()[0]
                .parse::<usize>()
                .unwrap();

            let clay_robot = line[1].split("Each clay robot costs ")
                .collect::<Vec<&str>>()[1]
                .split(" ore")
                .collect::<Vec<&str>>()[0]
                .parse::<usize>()
                .unwrap();

            let obsidian_robot_line = line[2].split("Each obsidian robot costs ")
                .collect::<Vec<&str>>()[1];

            let obsidian_robot_ore = obsidian_robot_line
                .split(" ore")
                .collect::<Vec<&str>>()[0]
                .parse::<usize>()
                .unwrap();

            let obsidian_robot_clay = obsidian_robot_line
                .split(" ore and ")
                .collect::<Vec<&str>>()[1]
                .split(" clay")
                .collect::<Vec<&str>>()[0]
                .parse::<usize>()
                .unwrap();

            let geode_robot_line = line[3].split("Each geode robot costs ")
                .collect::<Vec<&str>>()[1];

            let geode_robot_ore = geode_robot_line
                .split(" ore")
                .collect::<Vec<&str>>()[0]
                .parse::<usize>()
                .unwrap();

            let geode_robot_obsidian = geode_robot_line
                .split(" ore and ")
                .collect::<Vec<&str>>()[1]
                .split(" obsidian")
                .collect::<Vec<&str>>()[0]
                .parse::<usize>()
                .unwrap();

            Blueprint::new(
                index + 1,
                ore_robot,
                clay_robot,
                (obsidian_robot_ore, obsidian_robot_clay),
                (geode_robot_ore, geode_robot_obsidian),
            )
        })
        .collect();

    println!(
        "Solution to part 1: {:?}",
        solve(&blueprints, 24)
            .iter()
            .enumerate()
            .map(|(index, robots)| (index + 1) * robots)
            .sum::<usize>()
    );
    println!(
        "Solution to part 2: {:?}",
        solve(&blueprints.into_iter().take(3).collect(), 32)
            .iter()
            .fold(1, |acc, value| acc * value)
    );
}

fn solve(blueprints: &Vec<Blueprint>, timelimit: usize) -> Vec<usize> {
    let mut answer: Vec<usize> = vec![];

    for blueprint in blueprints {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();

        // The state consists of (#ore, #clay, #obsidian, #geodes), (#ore-robots, #clay-robots, #obsidian-robots, #geode-robots)
        let initial_state = (timelimit, (0, 0, 0, 0), (1, 0, 0, 0));
        queue.push_front(initial_state);

        let mut maximum_number_of_geods = 0;

        let maximum_ore_spend = blueprint.ore_robot + blueprint.clay_robot + blueprint.obsidian_robot.0 + blueprint.geode_robot.0;
        let maximum_clay_spend = blueprint.obsidian_robot.1;
        let maximum_obsidian_spend = blueprint.geode_robot.1;

        while let Some((remaining_time, (ores, clays, obsidian, geodes), (ore_robots, clay_robots, obsidian_robots, geode_robots))) = queue.pop_front() {
            maximum_number_of_geods = max(geodes, maximum_number_of_geods);

            if remaining_time == 0 {
                continue;
            }

            if seen.contains(&(remaining_time, (ores, clays, obsidian, geodes), (ore_robots, clay_robots, obsidian_robots, geode_robots))) {
                continue;
            }

            seen.insert((remaining_time, (ores, clays, obsidian, geodes), (ore_robots, clay_robots, obsidian_robots, geode_robots)));

            // Only wait, if we cannot built every robot already
            let mut could_wait = false;

            if remaining_time > 2 && maximum_ore_spend > ore_robots && ores >= blueprint.ore_robot {
                queue.push_front(
                    (
                        remaining_time - 1,
                        (
                            ores + ore_robots - blueprint.ore_robot,
                            clays + clay_robots,
                            obsidian + obsidian_robots,
                            geodes + geode_robots,
                        ),
                        (
                            ore_robots + 1,
                            clay_robots,
                            obsidian_robots,
                            geode_robots
                        )
                    )
                )
            } else {
                could_wait = true;
            }

            if remaining_time > 3 && maximum_clay_spend > clay_robots && ores >= blueprint.clay_robot {
                queue.push_back(
                    (
                        remaining_time - 1,
                        (
                            ores + ore_robots - blueprint.clay_robot,
                            clays + clay_robots,
                            obsidian + obsidian_robots,
                            geodes + geode_robots,
                        ),
                        (
                            ore_robots,
                            clay_robots + 1,
                            obsidian_robots,
                            geode_robots
                        )
                    )
                )
            } else {
                could_wait = true;
            }

            if remaining_time > 2 && maximum_obsidian_spend > obsidian_robots && ores >= blueprint.obsidian_robot.0 && clays >= blueprint.obsidian_robot.1 {
                queue.push_back(
                    (
                        remaining_time - 1,
                        (
                            ores + ore_robots - blueprint.obsidian_robot.0,
                            clays + clay_robots - blueprint.obsidian_robot.1,
                            obsidian + obsidian_robots,
                            geodes + geode_robots,
                        ),
                        (
                            ore_robots,
                            clay_robots,
                            obsidian_robots + 1,
                            geode_robots
                        )
                    )
                )
            } else {
                could_wait = true;
            }

            if remaining_time > 1 && ores >= blueprint.geode_robot.0 && obsidian >= blueprint.geode_robot.1 {
                queue.push_back(
                    (
                        remaining_time - 1,
                        (
                            ores + ore_robots - blueprint.geode_robot.0,
                            clays + clay_robots,
                            obsidian + obsidian_robots - blueprint.geode_robot.1,
                            geodes + geode_robots,
                        ),
                        (
                            ore_robots,
                            clay_robots,
                            obsidian_robots,
                            geode_robots + 1
                        )
                    )
                )
            } else {
                could_wait = true;
            }

            if could_wait {
                queue.push_front(
                    (
                        remaining_time - 1,
                        (
                            ores + ore_robots,
                            clays + clay_robots,
                            obsidian + obsidian_robots,
                            geodes + geode_robots,
                        ),
                        (
                            ore_robots,
                            clay_robots,
                            obsidian_robots,
                            geode_robots
                        )
                    )
                )
            }
        }
        println!("Maximum number of geodes for blueprint {:?}: {:?}", blueprint.id, maximum_number_of_geods);
        answer.push(maximum_number_of_geods);
    }

    answer
}