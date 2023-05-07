use crate::graph::Graph;
use std::collections::{HashMap, VecDeque};

pub type Component = usize;
//creating shortest path between the two randomly selected vertices and finding the degree of sep
pub fn bfs(graph: &Graph, source: &str, target: &str) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    visited.insert(source.to_string(), 0);
    queue.push_back(source.to_string());
    while !queue.is_empty() { // run until the queue is empty 
        let current = queue.pop_front().unwrap();
        if current == target {
            return visited.get(&current).cloned(); //reaching the target martix, returning the distance from it
        }
        if let Some(neighbors) = graph.get(&current) {
            for neighbor in neighbors { //if none, then looking at the current vertex and its neighbor
                if !visited.contains_key(neighbor) {
                    visited.insert(neighbor.to_string(), visited[&current] + 1); //add to visted if we haven't 'visted' but distane in + 1
                    queue.push_back(neighbor.to_string());
                }
            }
        }
    }
    None //else no connect
}

pub fn compute_connected_components(graph: &Graph) -> HashMap<Component, usize> {
    let mut component_counts: HashMap<Component, usize> = HashMap::new();
    let mut components: HashMap<String, Component> = HashMap::new();
    let mut component_id = 0;

    for node in graph.keys() { //iterarting through each vertex in the graph
        if !components.contains_key(node) {
            component_id += 1;
            mark_component_bfs(node, graph, &mut components, component_id);
        }
    }

    for component in components.values() {
        *component_counts.entry(*component).or_insert(0) += 1; // counting number of vertices for each componet
    }

    component_counts
}

pub fn six_degree_separation(graph: &Graph) -> Option<usize> { //finding SDS
    use rand::seq::IteratorRandom;
    let source = graph.keys().choose(&mut rand::thread_rng()).unwrap().to_string(); //random source and target
    let target = graph.keys().choose(&mut rand::thread_rng()).unwrap().to_string();
    bfs(graph, &source, &target) //calling to find the shortest distance between target vs soruce
}


fn mark_component_bfs( //aim of this function is to mark all the vertices that are connected
    start_node: &str,
    graph: &Graph,
    components: &mut HashMap<String, Component>, 
    component_id: Component,
) {
    let mut queue = VecDeque::new();
    components.insert(start_node.to_string(), component_id); // source to connected componenet
    queue.push_back(start_node.to_string());

    while let Some(current) = queue.pop_front() { //going through queue till empty
        if let Some(neighbors) = graph.get(&current) {
            for neighbor in neighbors { //looping for if no assigment in terms of component adds to queue
                if !components.contains_key(neighbor) {
                    components.insert(neighbor.to_string(), component_id);
                    queue.push_back(neighbor.to_string());
                }
            }
        }
    }
}