use std::cmp::min;
use std::collections::{BinaryHeap, HashSet};
use petgraph::algo::dijkstra;
use petgraph::{Directed, Graph, Undirected};
use petgraph::graph::{Node, UnGraph};
use petgraph::stable_graph::NodeIndex;

fn main() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut elevation_grid: Vec<Vec<usize>> = vec![];

    let mut graph: Graph<(), (), Directed> = Graph::new();
    let mut node_index_grid: Vec<Vec<NodeIndex>> = vec![];
    let mut source: Option<NodeIndex> = None;
    let mut sink: Option<NodeIndex> = None;

    for row in include_str!("input.txt").lines() {
        let mut elevation_row: Vec<usize> = vec![];
        let mut node_index_row: Vec<NodeIndex> = vec![];

        for element in row.chars() {
            let node = graph.add_node(());

            elevation_row.push(
                match element {
                    'S' => {
                        source = Some(node);
                        0
                    }
                    'E' => {
                        sink = Some(node);
                        25
                    }
                    _ => {alphabet.find(element).unwrap()}
                }
            );

            node_index_row.push(node);
        }

        elevation_grid.push(elevation_row);
        node_index_grid.push(node_index_row);
    }

    assert!(sink.is_some());
    assert!(source.is_some());

    for (x, row) in elevation_grid.iter().enumerate() {
        for (y, height) in row.iter().enumerate() {
            let current_node = node_index_grid[x][y];

            if x != elevation_grid.len() - 1 && can_traverse_from_height_to_height(*height, elevation_grid[x+1][y]) {
                graph.add_edge(current_node, node_index_grid[x+1][y], ());
            }
            if y != row.len() - 1 && can_traverse_from_height_to_height(*height, elevation_grid[x][y+1]) {
                graph.add_edge(current_node, node_index_grid[x][y+1], ());
            }
            if x > 0 && can_traverse_from_height_to_height(*height, elevation_grid[x-1][y]) {
                graph.add_edge(current_node, node_index_grid[x-1][y], ());
            }
            if y > 0 && can_traverse_from_height_to_height(*height, elevation_grid[x][y-1]) {
                graph.add_edge(current_node, node_index_grid[x][y-1], ());
            }
        }
    }

    let result = dijkstra(&graph, source.unwrap(), sink, |_| 1);
    println!("Solution to part 1: {:?}", result.get(&sink.unwrap()));
    
    let mut best = None;
    for (x, row) in elevation_grid.iter().enumerate() {
        for (y, height) in row.iter().enumerate() {
            if *height != 0 {
                continue;
            }

            let result = dijkstra(&graph, node_index_grid[x][y], sink, |_| 1);
            if let Some(distance) = result.get(&sink.unwrap()) {
                best = Some(min(distance.clone(), best.unwrap_or(distance.clone())));
            }
        }
    }

    println!("Solution to part 2: {:?}", best);
}

fn can_traverse_from_height_to_height(height: usize, other_height: usize) -> bool {
    if height >= other_height {
        return true;
    }

    if height + 1 == other_height {
        return true;
    }

    return false;
}
