mod aco;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;
fn main() {
    println!("Hello, world!");
    let f = File::open("../../cities.csv").unwrap();
    let file = BufReader::new(&f);
    let mut distance: Vec<aco::Coordinate> = Vec::new();
    for line in file.lines() {
        let line = line.unwrap();
        let line = line.trim();

        let line: Vec<&str> = line.split(",").collect();
        distance.push(aco::Coordinate { x: line[1].parse().unwrap(), y: line[2].parse().unwrap()});
    }

    let mut antcolony = aco::AntColony{
    no_ants: 4,
    no_best_paths: 100,
    no_iterations: 200,
    decay: None,
    alpha: 1.0,
    beta: 1.0,
    distances: distance,
    initial_tour: None,
    default_pheromone: 0.0,
    nodes: Vec::new(),
    pheromones: HashMap::new(),
    };
    println!("{:?}", antcolony.run());

}
