extern crate aco_rust;
extern crate fnv;
use fnv::FnvHashMap;
use aco_rust::*;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("Hello, world!");
    // let f = File::open("/home/datascience/Documents/aco_rust/cities.csv").unwrap();
    let f = File::open("cities.csv").unwrap();
    let file = BufReader::new(&f);
    let mut distance: Vec<aco_no_mem::Coordinate> = Vec::new();
    for line in file.lines() {
        let line = line.unwrap();
        let line = line.trim();
        let line: Vec<&str> = line.split(",").collect();
        distance.push(aco_no_mem::Coordinate {
            x: line[1].parse().unwrap(),
            y: line[2].parse().unwrap(),
        });
    }
    let f = File::open("submission.csv").unwrap();
    let file = BufReader::new(&f);
    let mut initial_tour: Vec<i32> = Vec::new();
    for line in file.lines() {
        let line = line.unwrap();
        let line = line.trim();
        initial_tour.push(line.parse().unwrap());
    }

    let mut antcolony = aco_no_mem::AntColony {
        no_ants: 10,
        no_best_paths: 100,
        no_iterations: 100,
        decay: None,
        alpha: 1.0,
        beta: 1.0,
        distances: distance,
        // initial_tour: None,
        initial_tour: Some(initial_tour),
        default_pheromone: 0.0,
        nodes: Vec::new(),
        pheromones: FnvHashMap::default(),
        //pheromones: HashMap::default(),
    };
    println!("{:?}", antcolony.run());
}
