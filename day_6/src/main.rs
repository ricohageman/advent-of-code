use std::collections::HashSet;

fn execute(length: usize) -> usize {
    include_str!("input.txt")
        .as_bytes()
        .windows(window_length)
        .enumerate()
        .filter(|(_, window)| HashSet::<u8>::from_iter(window.iter().cloned()).len() == window_length)
        .next()
        .map(|(index, _)| index + window_length)
        .unwrap()
}

fn main() {
    println!("Solution to part 1: {:?}", execute(4));
    println!("Solution to part 2: {:?}", execute(14));
}
