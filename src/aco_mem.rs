extern crate permutation;
extern crate rand;
extern crate rayon;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::f64::MAX;
use std::f64::INFINITY;

#[derive(Debug, Clone)]
pub struct F64(pub f64);

impl PartialEq for F64 {
    fn eq(&self, other: &F64) -> bool {
        if self.0 == other.0 {
            true
        } else {
            false
        }
    }
}

impl PartialOrd for F64 {
    fn partial_cmp(&self, other: &F64) -> Option<Ordering> {
        if self.0 > other.0 {
            Some(Ordering::Greater)
        } else if self.0 < other.0 {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Eq for F64 {}

impl Ord for F64 {
    fn cmp(&self, other: &F64) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Default, Debug, Clone)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

#[derive(Default, Debug)]
pub struct AntColony {
    pub no_ants: i64,
    pub no_best_paths: i64,
    pub no_iterations: i64,
    pub decay: Option<i64>,
    pub alpha: f64,
    pub beta: f64,
    pub distances: Vec<Vec<f64>>,
    pub nodes: Vec<i64>,
    pub initial_tour: Option<Vec<i64>>,
    pub pheromones: Vec<Vec<f64>>,
    pub default_pheromone: f64,
}

#[derive(Default, Debug, Clone)]
pub struct UnitPath {
    pub start: i64,
    pub end: i64,
}

#[derive(Default, Debug)]
pub struct TourPath {
    pub path: Vec<UnitPath>,
    pub distance: f64,
}

impl Clone for TourPath {
    fn clone(&self) -> TourPath {
        TourPath {
            path: self.path.clone(),
            distance: self.distance,
        }
    }
}

impl TourPath {
    fn new() -> Self {
        TourPath {
            path: Vec::new(),
            distance: 0.0,
        }
    }
}

impl AntColony {
    pub fn run(&mut self) -> TourPath {
        for i in 0..self.distances.len() {
            self.nodes.push(i as i64);
        }
        self.pheromones = Vec::new();
        self.default_pheromone = 1.0 / (self.distances.len() as f64);

        for row in 0..self.distances.len() {
            let mut row_values: Vec<f64> = Vec::new();
            for _value in 0..self.distances[row].len() {
                row_values.push(self.default_pheromone);
            }
            self.pheromones.push(row_values);
        }
        //let mut shortest_path_iteration = TourPath::new();
        let mut shortest_path_all_time = TourPath::new();
        shortest_path_all_time.distance = MAX;
        for iteration in 0..self.no_iterations {
            println!("inside iteration no {}", iteration);
            let mut iteration_paths: Vec<TourPath>;
            iteration_paths = self.gen_paths();
            self.spread_pheromone(&iteration_paths, self.no_best_paths);
            let mut shortest_path_iteration = &iteration_paths[0];
            for iteration_path in &iteration_paths {
                if iteration_path.distance < shortest_path_iteration.distance {
                    shortest_path_iteration = iteration_path;
                }
            }
            //println!("----------------------------");
            //println!("{} {:?}", iteration, shortest_path_iteration);
            if shortest_path_iteration.distance < shortest_path_all_time.distance {
                shortest_path_all_time = shortest_path_iteration.clone();
            }
        }
        return shortest_path_all_time;
    }

    fn gen_paths(&self) -> Vec<TourPath> {
        //println!("inside genpaths");
        return (0..self.no_ants)
            .collect::<Vec<i64>>()
            .iter()
            .map(|_x| self.gen_path(0))
            .map(|x| TourPath {
                distance: self.path_distance(&x),
                path: x,
            })
            .collect();
    }

    fn gen_path(&self, start: i64) -> Vec<UnitPath> {
        //println!("inside genpath");
        let mut antpath: Vec<UnitPath> = Vec::new();
        let mut visited_nodes: HashSet<i64> = HashSet::new();
        visited_nodes.insert(start);
        let mut prev = start;
        for i in 0..(self.distances.len() - 1) {
            //if i % 100 == 0 {
            //println!("inside distance in genpath {}", i);
            //}
            let pheromone_prev: Vec<f64> = self.pheromones[prev as usize].clone();

            //println!("pheromones ----------> {:?}", &pheromone_prev);
            let next = self.pick_next_move(
                pheromone_prev,
                &self.distances[prev as usize],
                &visited_nodes,
            );
            antpath.push(UnitPath {
                start: prev,
                end: next,
            });
            prev = next;
            visited_nodes.insert(next);
        }
        antpath.push(UnitPath {
            start: prev,
            end: start,
        });
        return antpath;
    }

    fn pick_next_move(
        &self,
        mut pheromone: Vec<f64>,
        distances: &Vec<f64>,
        visited: &HashSet<i64>,
    ) -> i64 {
        //println!("pheromone------> {:?}", pheromone);
        //println!("visited------> {:?}", visited);
        for i in visited {
            pheromone[*i as usize] = 0.0;
        }

        let mut row: Vec<f64> = Vec::new();

        for i in 0..pheromone.len() {
            //into is for converting between f64 and f64
            let value = (pheromone[i]).powf(self.alpha) * (1.0 / distances[i]).powf(self.beta);
            //let value = (pheromone[i]) * (1.0 / distances[i]);
            row.push(value);
        }

        //println!("row---------->{:?}", row);
        let row_sum: f64 = row.iter().sum();

        //println!("row_sum---------->{:?}", row_sum);
        for i in 0..row.len() {
            row[i] = row[i] / row_sum;
        }

        //println!("row---------->{:?}", row);
        let dist = WeightedIndex::new(&row).unwrap();
        let mut rng = thread_rng();
        self.nodes[dist.sample(&mut rng)]
    }

    fn path_distance(&self, iteration_path: &Vec<UnitPath>) -> f64 {
        let mut distance = 0.0;
        for edge in iteration_path {
            distance += self.distances[edge.start as usize][edge.end as usize];
        }
        return distance;
    }

    fn spread_pheromone(&mut self, iteration_paths: &Vec<TourPath>, no_best_path: i64) {
        let permutation = permutation::sort(
            &iteration_paths
                .iter()
                .map(|x| F64(x.distance))
                .collect::<Vec<F64>>()[..],
        );
        let ordered_paths = permutation.apply_slice(&iteration_paths[..]);
        let mut count = 0;
        for path in ordered_paths {
            if count == no_best_path {
                break;
            }
            count += 1;

            for edge in path.path {
                self.pheromones[edge.start as usize][edge.end as usize] = self.pheromones
                    [edge.start as usize][edge.end as usize]
                    + 1.0 / (self.distances[edge.start as usize][edge.end as usize])
            }
        }
    }

    fn euclidean_distance(a: &Coordinate, b: &Coordinate) -> f64 {
        //let distance = ((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).sqrt();
        let distance = ((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)).sqrt();
        if distance == 0.0 {
            return std::f64::INFINITY;
        } else {
            return distance;
        }
    }
}
