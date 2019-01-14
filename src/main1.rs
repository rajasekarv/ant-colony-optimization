mod aco_mem;
//mod aco_no_mem;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::f64::INFINITY;

//fn main() {
//println!("Hello, world!");
//let f = File::open("../../cities.csv").unwrap();
//let file = BufReader::new(&f);
//let mut distance: Vec<aco_no_mem::Coordinate> = Vec::new();
//for line in file.lines() {
//let line = line.unwrap();
//let line = line.trim();
//
//let line: Vec<&str> = line.split(",").collect();
//distance.push(aco_no_mem::Coordinate { x: line[1].parse().unwrap(), y: line[2].parse().unwrap()});
//}
//
//let mut antcolony = aco_no_mem::AntColony{
//no_ants: 4,
//no_best_paths: 100,
//no_iterations: 200,
//decay: None,
//alpha: 1.0,
//beta: 1.0,
//distances: distance,
//initial_tour: None,
//default_pheromone: 0.0,
//nodes: Vec::new(),
//pheromones: HashMap::new(),
//};
//println!("{:?}", antcolony.run());
//
//}

fn main() {
    println!("Hello, world!");
    let f = File::open(
        "/home/raja/Downloads/programming/python/AntColonyOptimization-master/att48_d.txt",
    )
    .unwrap();
    let file = BufReader::new(&f);
    let mut distance: Vec<Vec<f64>> = Vec::new();
    for line in file.lines() {
        let line = line.unwrap();
        let line = line.trim();
        let line: Vec<&str> = line.split("      ").collect();
        distance.push(
            line.iter()
                .map(|x| {
                    let dis = x.trim().parse().unwrap();
                    if dis == 0.0{
                        INFINITY                        
                    }else{
                        dis
                    }
                })
                .collect(),
        );
    }

    //println!("{:?}", distance);

    let mut antcolony = aco_mem::AntColony {
        no_ants: 200,
        no_best_paths: 100,
        no_iterations: 200,
        decay: None,
        alpha: 1.0,
        beta: 1.0,
        distances: distance,
        initial_tour: None,
        default_pheromone: 0.0,
        nodes: Vec::new(),
        pheromones: Vec::new(),
    };
    println!("{:?}", antcolony.run());
}
