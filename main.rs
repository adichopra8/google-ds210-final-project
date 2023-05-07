mod graph;
mod bfs;

use bfs::{bfs, six_degree_separation};
use graph::{parse_file, random_vertex};
use std::io;

fn main() {
    let filename = "gplus_combined.txt";
    let graph = parse_file(filename); //parse graph from file
    let component_counts = bfs::compute_connected_components(&graph);
    println!("{} components found.", component_counts.len()); //amount of componets
    let edge_count = graph::count_edges(&graph);
    println!("{} edges found.", edge_count); //amount of edges
    for (component_id, count) in component_counts.iter() {
        println!("Component {}: {} nodes", component_id, count); //amount of nodes in each componets
    }

    println!("Six-degree separation:");
    for i in 1..6{ // loop to run on 5 differnt pairs of nodes
        println!("Test {}:", i);
        let source = random_vertex(&graph); //random id starting
        let target = random_vertex(&graph); //random id ending
        println!("Random Source: {}", source);
        println!("Random Target: {}", target);

        if let Some(degree) = bfs(&graph, &source, &target) { // call bfs to find the degree of sep between starting and ending
            if degree <= 6 {
                println!(
                    "The degree of separation between {} and {} is {}. They are in six degrees of separation.", source, target, degree);
        }   else {
                println!(
                    "The degree of separation between {} and {} is {}. They are not within six degrees of separation.", source, target, degree);
        }
    }   else {
            println!("There is no path between {} and {}", source, target);
    }
  
    }
}
