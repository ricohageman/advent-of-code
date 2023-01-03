fn main() {
    println!(
        "Solution to part 1: {:?}",
        include_str!("input.txt")
            .lines()
            .map(|n| {
                let parsed_string = n.parse::<String>().unwrap();
                let mut lines = parsed_string.split_whitespace();
                let opponent = lines.next().unwrap();
                let recommendation = lines.next().unwrap();

                let selected_score = match recommendation {
                    "X" => 1,
                    "Y" => 2,
                    "Z" => 3,
                    &_ => panic!("{:?}", recommendation)
                };

                let outcome_score = match (opponent, recommendation) {
                    ("A", "Z") => 0,
                    ("B", "X") => 0,
                    ("C", "Y") => 0,
                    ("A", "X") => 3,
                    ("B", "Y") => 3,
                    ("C", "Z") => 3,
                    ("A", "Y") => 6,
                    ("B", "Z") => 6,
                    ("C", "X") => 6,
                    (_, _) => panic!("{:?}", (opponent, recommendation))
                };

                outcome_score + selected_score
            })
            .sum::<usize>()
    );

    println!(
        "Solution to part 2: {:?}",
        include_str!("input.txt")
            .lines()
            .map(|n| {
                let parsed_string = n.parse::<String>().unwrap();
                let mut lines = parsed_string.split_whitespace();
                let opponent = lines.next().unwrap();
                let target_outcome = lines.next().unwrap();

                let recommended_action = match (opponent, target_outcome) {
                    ("A", "X") => "Z",
                    ("A", "Y") => "X",
                    ("A", "Z") => "Y",
                    ("B", "X") => "X",
                    ("B", "Y") => "Y",
                    ("B", "Z") => "Z",
                    ("C", "X") => "Y",
                    ("C", "Y") => "Z",
                    ("C", "Z") => "X",
                    _ => panic!("{:?}", (opponent, target_outcome))
                };

                let selected_score = match recommended_action {
                    "X" => 1,
                    "Y" => 2,
                    "Z" => 3,
                    &_ => panic!("{:?}", recommended_action)
                };

                let outcome_score = match target_outcome {
                    "X" => 0,
                    "Y" => 3,
                    "Z" => 6,
                    &_ => panic!("{:?}", target_outcome)
                };

                outcome_score + selected_score
            })
            .sum::<usize>()
    );
}
