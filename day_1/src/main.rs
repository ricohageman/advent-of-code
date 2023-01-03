fn main() {
    println!(
        "Solution to part 1: {:?}",
        include_str!("input.txt")
            .lines()
            .map(|n| n.parse::<String>().unwrap())
            .fold(
                Vec::new(),
                |mut acc: Vec<usize>, x| {
                    if x == "" || acc.is_empty() {
                        acc.push(0);
                    }

                    if x != "" {
                        *acc.last_mut().unwrap() += x.parse::<usize>().unwrap();
                    }

                    acc
                }
            )
            .iter()
            .max()
    );

    println!(
        "Solution to part 2: {:?}",
        include_str!("input.txt")
            .lines()
            .map(|n| n.parse::<String>().unwrap())
            .fold(
                Vec::new(),
                |mut acc: Vec<usize>, x| {
                    if x == "" || acc.is_empty() {
                        acc.push(0);
                    }

                    if x != "" {
                        *acc.last_mut().unwrap() += x.parse::<usize>().unwrap();
                    }

                    acc
                }
            )
            .iter()
            .sorted_by(|a, b| b.cmp(a))
            .take(3)
            .sum::<usize>()
    );
}
