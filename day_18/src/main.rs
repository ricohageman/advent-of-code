use std::collections::{HashSet, VecDeque};

fn sides_of_point(point: &[isize; 3]) -> [[isize; 3]; 6] {
    let [x, y, z] = point.clone();

    [
        [x - 1, y, z],
        [x + 1, y, z],
        [x, y - 1, z],
        [x, y + 1, z],
        [x, y, z - 1],
        [x, y, z + 1],
    ]
}

fn main() {
    let points = include_str!("input.txt")
        .lines()
        .map(|line| {
            let mut splitted_line = line.split(",");
            [
                splitted_line.next().unwrap().parse().unwrap(),
                splitted_line.next().unwrap().parse().unwrap(),
                splitted_line.next().unwrap().parse().unwrap(),
            ]
        })
        .collect::<HashSet<[isize; 3]>>();

    println!("Solution to part 1: {:?}", points
        .iter()
        .map(|point| sides_of_point(point).iter().filter(|point| !points.contains(*point)).count())
        .sum::<usize>()
    );

    let points: Vec<[isize; 3]> = points.iter().map(|[x, y, z]| [*x+1, *y+1, *z+1]).collect();

    let x_max = points.iter().map(|[x, _, _]| x).max().unwrap().clone() + 1;
    let y_max = points.iter().map(|[_, y, _]| y).max().unwrap().clone() + 1;
    let z_max = points.iter().map(|[_, _, z]| z).max().unwrap().clone() + 1;

    let mut queue: VecDeque<[isize; 3]> = VecDeque::from([[0, 0, 0]]);
    let mut visited: HashSet<[isize; 3]> = HashSet::new();
    let mut droplet_faces = 0;

    while let Some(point) = queue.pop_front() {
        let neighbouring_points = sides_of_point(&point)
            .into_iter()
            .filter(|[x, y, z]| *x <= x_max && *x >= 0 && *y <= y_max && *y >= 0 && *z <= z_max && *z >= 0)
            .collect::<Vec<[isize; 3]>>();

        // Count how many of these are actually faces of the droplet
        droplet_faces += neighbouring_points.iter().filter(|point| points.contains(*point)).count();

        let mut next_points = neighbouring_points
            .into_iter()
            .filter(|point| !points.contains(point))
            .filter(|point| !visited.contains(point))
            .filter(|point| !queue.contains(point))
            .collect();

        queue.append(&mut next_points);
        visited.insert(point);
    }

    println!("Solution to part 2: {:?}", droplet_faces);
}
