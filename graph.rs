use rand::seq::IteratorRandom;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub type Graph = HashMap<String, Vec<String>>;

pub fn parse_file(filename: &str) -> Graph {
    let mut graph = Graph::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect(); //separating list with whitespace given each line
        if parts.len() == 2 { // 2 parts, adding edge 
            let user1 = parts[0].to_string(); // col 1 
            let user2 = parts[1].to_string(); // col 2 
            graph.entry(user1.clone()).or_insert(Vec::new()).push(user2.clone());
            graph.entry(user2).or_insert(Vec::new()).push(user1);
        }
    }
    graph
}
//selecting random vector from the created graph, this is the testing componet running random vectors to find degree of sep
pub fn random_vertex(graph: &Graph) -> String {
    graph.keys().choose(&mut rand::thread_rng()).unwrap().to_string()
}

pub fn count_edges(graph: &Graph) -> usize {
    let mut edge_count = 0;
    for (_node, neighbors) in graph {
        edge_count += neighbors.len();
    }
    edge_count/2 // each edge is counted twice
}
