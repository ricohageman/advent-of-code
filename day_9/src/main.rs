use std::collections::HashSet;

fn execute(number_of_knots: usize) -> usize {
    let mut visited_positions: HashSet<(isize, isize)> = HashSet::new();
    let mut rope = vec![(0, 0); number_of_knots];

    for line in include_str!("input.txt").lines() {
        let mut data = line.split_whitespace();
        let direction = data.next().unwrap();
        let number_of_steps = data.next().unwrap().parse::<usize>().unwrap();

        for _ in 0..number_of_steps {
            let mut rope_clone = rope.clone();

            match direction {
                "R" => rope_clone[0].0 += 1,
                "L" => rope_clone[0].0 -= 1,
                "U" => rope_clone[0].1 += 1,
                "D" => rope_clone[0].1 -= 1,
                _ => panic!("{direction}")
            }

            for index in 1..rope_clone.len() {
                let x_difference: isize = rope_clone[index - 1].0 - rope[index].0;
                let y_difference: isize = rope_clone[index - 1].1 - rope[index].1;

                if x_difference.abs() == 2 || y_difference.abs() == 2 {
                    rope_clone[index].0 += x_difference.signum();
                    rope_clone[index].1 += y_difference.signum();
                }
            }

            visited_positions.insert(*rope_clone.last().unwrap());
            rope = rope_clone;
        }
    }

    visited_positions.len()
}

fn main() {
    println!("Solution to part 1: {:?}", execute(2));
    println!("Solution to part 2: {:?}", execute(10));
}
