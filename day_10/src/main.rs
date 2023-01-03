fn is_pixel_drawn(cycle: isize, register_value: isize) -> bool {
    let horizontal_pixel = cycle % 40;
    let lower_bound = register_value - 1;
    let upper_bound = register_value + 1;

    horizontal_pixel <= upper_bound && lower_bound <= horizontal_pixel
}

fn main() {
    let mut cycle = 1;
    let mut register_value = 1;

    let mut signal_strength = 0;
    let mut screen: Vec<bool> = vec![false; 240];

    for line in include_str!("input.txt").lines() {
        let mut data = line.split_whitespace();

        if (cycle - 20) % 40 == 0 {
            signal_strength += cycle * register_value;
        }

        screen[(cycle - 1) as usize] = is_pixel_drawn(cycle - 1, register_value);

        let (first, second) = (data.next(), data.next());
        match (first, second) {
            (Some("noop"), None) => {
                cycle += 1;
            }
            (Some("addx"), Some(amount)) => {
                if (cycle - 19) % 40 == 0 {
                    signal_strength += (cycle + 1) * register_value;
                }

                cycle += 1;
                screen[(cycle - 1) as usize] = is_pixel_drawn(cycle - 1, register_value);

                cycle += 1;

                register_value += amount.parse::<isize>().unwrap();
            }
            _ => {panic!("{:?} {:?}", first, second);}
        }
    }

    println!("Solution to part 1: {signal_strength}");
    println!("Solution to part 2");

    for nth_row in 0..6 {
        println!(
            "{}",
            (nth_row * 40..(nth_row + 1) * 40)
                .map(|index| {
                    match screen[index] {
                        true => "#",
                        false => " ",
                    }
                })
                .collect::<Vec<&str>>()
                .join(" ")
        );
    }
}
