
fn main() {
    println!(
        "Solution to part 1: {:?}",
        include_str!("input.txt")
            .lines()
            .filter(|n| {
                let parsed_string = n.parse::<String>().unwrap();
                let ranges = parsed_string.split(",")
                    .map(|range_string| {
                        let range = range_string.split("-")
                            .map(|element| element.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>();

                        assert!(range[0] <= range[1]);

                        (range[0], range[1])
                    })
                    .collect::<Vec<(usize, usize)>>();

                let (first_start, first_end) = ranges[0];
                let (second_start, second_end) = ranges[1];

                (first_start <= second_start && first_end >= second_end) || (second_start <= first_start && second_end >= first_end)
            })
            .count()
    );

    println!(
        "Solution to part 2: {:?}",
        include_str!("input.txt")
            .lines()
            .filter(|n| {
                let parsed_string = n.parse::<String>().unwrap();
                let ranges = parsed_string.split(",")
                    .map(|range_string| {
                        let range = range_string.split("-")
                            .map(|element| element.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>();

                        assert!(range[0] <= range[1]);

                        (range[0], range[1])
                    })
                    .collect::<Vec<(usize, usize)>>();

                let (first_start, first_end) = ranges[0];
                let (second_start, second_end) = ranges[1];

                second_start <= first_end && first_start <= second_end
            })
            .count()
    );
}
