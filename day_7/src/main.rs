use std::collections::HashMap;
use petgraph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::visit::{Dfs, Bfs};

fn main() {
    let lines = include_str!("input.txt").lines();
    let mut filesystem = Graph::new();
    let mut stack: Vec<NodeIndex> = vec![];
    let mut root: Option<NodeIndex> = None;
    let mut filesizes_per_directory = HashMap::<NodeIndex, usize>::new();

    for line in lines {
        let elements = line.split_whitespace().collect::<Vec<&str>>();

        match (elements[0], elements[1], elements.get(2)) {
            ("$", "ls", None) => {}
            ("$", "cd", Some(&"..")) => {stack.pop();}
            ("$", "cd", Some(&path)) => {
                let new_node = filesystem.add_node(path);

                if root.is_none() {
                    root = Some(new_node.clone());
                }

                if let Some(parent) = stack.last() {
                    filesystem.add_edge(*parent, new_node, 1);
                }

                stack.push(new_node);
            }
            ("dir", _, _) => {}
            (file_size, _, _) => {
                let current_directory = stack.last().unwrap();

                if !filesizes_per_directory.contains_key(current_directory) {
                    filesizes_per_directory.insert(*current_directory, 0);
                }


                let file_size = file_size.parse::<usize>().unwrap();
                *filesizes_per_directory.get_mut(current_directory).unwrap() += file_size
            }
        }
    }

    // Part A
    let mut total_sum: usize = 0;
    let mut dfs = Dfs::new(&filesystem, root.unwrap());
    while let Some(current_node) = dfs.next(&filesystem) {
        let mut sum_of_current_directory: usize = 0;

        let mut internal_dfs = Dfs::new(&filesystem, current_node);
        while let Some(current_internal_node) = internal_dfs.next(&filesystem) {
            sum_of_current_directory += *filesizes_per_directory.get(&current_internal_node).unwrap_or(&0);
        }

        if sum_of_current_directory <= 100000 {
            total_sum += sum_of_current_directory;
        }
    }

    println!("Solution to part 1: {:?}", total_sum);

    // Part B
    let memory_usage: usize = filesizes_per_directory.values().sum();
    let mut smallest_directory_to_remove= usize::MAX;

    let mut dfs = Dfs::new(&filesystem, root.unwrap());
    while let Some(current_node) = dfs.next(&filesystem) {
        let mut sum_of_current_directory: usize = 0;

        let mut internal_dfs = Dfs::new(&filesystem, current_node);
        while let Some(current_internal_node) = internal_dfs.next(&filesystem) {
            sum_of_current_directory += *filesizes_per_directory.get(&current_internal_node).unwrap_or(&0);
        }

        if memory_usage - sum_of_current_directory < 40000000 {
            if smallest_directory_to_remove > sum_of_current_directory {
                smallest_directory_to_remove = sum_of_current_directory
            }
        }
    }

    println!("Solution to part 2: {:?}", smallest_directory_to_remove);
}
