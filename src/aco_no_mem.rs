extern crate fnv;
extern crate permutation;
extern crate rand;
extern crate rayon;

use fnv::FnvHashMap;
use fnv::FnvHashSet;
use fnv::FnvHasher;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::f32::MAX;
use std::hash::BuildHasherDefault;

#[derive(Debug, Clone)]
pub struct F32(pub f32);

impl PartialEq for F32 {
    fn eq(&self, other: &F32) -> bool {
        if self.0 == other.0 {
            true
        } else {
            false
        }
    }
}

impl PartialOrd for F32 {
    fn partial_cmp(&self, other: &F32) -> Option<Ordering> {
        if self.0 > other.0 {
            Some(Ordering::Greater)
        } else if self.0 < other.0 {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Eq for F32 {}

impl Ord for F32 {
    fn cmp(&self, other: &F32) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Default, Debug, Clone)]
pub struct Coordinate {
    pub x: f32,
    pub y: f32,
}

#[derive(Default, Debug)]
pub struct AntColony {
    pub no_ants: i32,
    pub no_best_paths: i32,
    pub no_iterations: i32,
    pub decay: Option<i32>,
    pub alpha: f32,
    pub beta: f32,
    pub distances: Vec<Coordinate>,
    pub nodes: Vec<i32>,
    pub initial_tour: Option<Vec<i32>>,
    pub pheromones: FnvHashMap<i32, FnvHashMap<i32, f32>>,
    pub default_pheromone: f32,
}

#[derive(Default, Debug, Clone)]
pub struct UnitPath {
    pub start: i32,
    pub end: i32,
}

#[derive(Default, Debug)]
pub struct TourPath {
    pub path: Vec<UnitPath>,
    pub distance: f32,
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
            self.nodes.push(i as i32);
        }
        self.pheromones = FnvHashMap::default();
        self.default_pheromone = 1.0 / (self.distances.len() as f32);
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
            println!("----------------------------");
            println!("{} {:?}", iteration, shortest_path_iteration);
            if shortest_path_iteration.distance < shortest_path_all_time.distance {
                shortest_path_all_time = shortest_path_iteration.clone();
            }
        }
        return shortest_path_all_time;
    }

    fn gen_paths(&self) -> Vec<TourPath> {
        println!("inside genpaths");
        return (0..self.no_ants)
            .collect::<Vec<i32>>()
            .par_iter()
            .map(|_x| self.gen_path(0))
            .map(|x| TourPath {
                distance: self.path_distance(&x),
                path: x,
            })
            .collect();
    }

    fn gen_path(&self, start: i32) -> Vec<UnitPath> {
        println!("inside genpath");
        let mut antpath: Vec<UnitPath> = Vec::new();
        let mut visited_nodes: Vec<bool> = (0..self.distances.len())
            .into_iter()
            .map(|_x| false)
            .collect();
        visited_nodes[start as usize] = true;
        let mut prev = start;
        for i in 0..(self.distances.len() - 1) {
            if i % 1000 == 0 {
                println!("inside distance in genpath {}", i);
            }
            let next = self.pick_next_move(prev, &visited_nodes);
            antpath.push(UnitPath {
                start: prev,
                end: next,
            });
            prev = next;
            visited_nodes[next as usize] = true;
        }
        antpath.push(UnitPath {
            start: prev,
            end: start,
        });
        return antpath;
    }

    fn pick_next_move(&self, prev: i32, visited: &Vec<bool>) -> i32 {
        let row: Vec<f32> = match self.pheromones.get(&prev) {
            Some(val) => (0..self.distances.len())
                .into_iter()
                .map(|j| match val.get(&(j as i32)) {
                    Some(val1) => {
                        if visited[j] {
                            0.0
                        } else {
                            *val1
                                * (1.0
                                    / AntColony::euclidean_distance(
                                        &self.distances[prev as usize],
                                        &self.distances[j],
                                    ))
                        }
                    }
                    None => {
                        if visited[j] {
                            0.0
                        } else {
                            self.default_pheromone
                                * (1.0
                                    / AntColony::euclidean_distance(
                                        &self.distances[prev as usize],
                                        &self.distances[j],
                                    ))
                        }
                    }
                })
                .collect(),
            None => (0..self.distances.len())
                .into_iter()
                .map(|j| {
                    ({
                        if visited[j] {
                            0.0
                        } else {
                            self.default_pheromone
                                * (1.0
                                    / AntColony::euclidean_distance(
                                        &self.distances[prev as usize],
                                        &self.distances[j],
                                    ))
                        }
                    })
                })
                .collect(),
        };
        let dist = WeightedIndex::new(&row).unwrap();
        let mut rng = thread_rng();
        self.nodes[dist.sample(&mut rng)]
    }

    //fn gen_path(&self, start: i32) -> Vec<UnitPath> {
    //println!("inside genpath");
    //let mut antpath: Vec<UnitPath> = Vec::new();
    //let mut visited_nodes: FnvHashSet<i32> = FnvHashSet::default();
    //visited_nodes.insert(start);
    //let mut prev = start;
    //let mut distances: Vec<f32> = Vec::new();
    ////let mut pheromone_prev: Vec<f32> = Vec::new();
    //let mut pheromone_prev: Vec<f32> = (0..self.distances.len())
    //.into_iter()
    //.map(|_x| self.default_pheromone)
    //.collect();
    //for i in 0..(self.distances.len() - 1) {
    //if i % 100 == 0 {
    //println!("inside distance in genpath {}", i);
    //}
    ////let pheromone_prev: Vec<f32> = match self.pheromones.get(&prev) {
    //pheromone_prev.clear();
    //pheromone_prev = match self.pheromones.get(&prev) {
    //Some(val) => (0..self.distances.len())
    //.into_iter()
    //.map(|j| {
    //match val.get(&(j as i32)) {
    //Some(val1) => *val1,
    //None => self.default_pheromone,
    //}
    ////.to_owned()
    //})
    //.collect(),
    //None => (0..self.distances.len())
    //.into_iter()
    //.map(|_j| (self.default_pheromone))
    //.collect(),
    //};
    //
    //distances.clear();
    //for k in &self.distances {
    //distances.push(AntColony::euclidean_distance(
    //&self.distances[prev as usize],
    //k,
    //));
    //}
    //let next = self.pick_next_move(&mut pheromone_prev, &distances, &visited_nodes);
    //antpath.push(UnitPath {
    //start: prev,
    //end: next,
    //});
    //prev = next;
    //visited_nodes.insert(next);
    //}
    //antpath.push(UnitPath {
    //start: prev,
    //end: start,
    //});
    //return antpath;
    //}
    //
    //fn pick_next_move(
    //&self,
    //pheromone: &mut Vec<f32>,
    //distances: &Vec<f32>,
    //visited: &FnvHashSet<i32>,
    //) -> i32 {
    //for i in visited {
    //pheromone[*i as usize] = 0.0;
    //}
    //
    //let mut row: Vec<f32> = Vec::with_capacity(pheromone.len());
    //
    //for i in 0..pheromone.len() {
    ////into is for converting between f32 and f32
    ////let value = (pheromone[i]).powf(self.alpha) * (1.0 / distances[i]).powf(self.beta);
    //let value = (pheromone[i]) * (1.0 / distances[i]);
    //row.push(value);
    //}
    //
    //let dist = WeightedIndex::new(&row).unwrap();
    //let mut rng = thread_rng();
    //self.nodes[dist.sample(&mut rng)]
    //}

    fn path_distance(&self, iteration_path: &Vec<UnitPath>) -> f32 {
        iteration_path
            .iter()
            .map(|x| {
                AntColony::euclidean_distance(
                    &self.distances[x.start as usize],
                    &self.distances[x.end as usize],
                )
            })
            .sum()
    }

    fn spread_pheromone(&mut self, iteration_paths: &Vec<TourPath>, no_best_path: i32) {
        let permutation = permutation::sort(
            &iteration_paths
                .iter()
                .map(|x| F32(x.distance))
                .collect::<Vec<F32>>()[..],
        );
        let ordered_paths = permutation.apply_slice(&iteration_paths[..]);
        let mut count = 0;
        for path in ordered_paths {
            if count == no_best_path {
                break;
            }
            count += 1;

            for edge in path.path {
                let inb = self
                    .pheromones
                    .entry(edge.start)
                    .or_insert(FnvHashMap::default());
                *inb.entry(edge.end).or_insert(self.default_pheromone) += 1.0
                    / (AntColony::euclidean_distance(
                        &self.distances[edge.start as usize],
                        &self.distances[edge.end as usize],
                    ));
            }
        }
    }

    fn euclidean_distance(a: &Coordinate, b: &Coordinate) -> f32 {
        //let distance = ((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).sqrt();
        let distance = ((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)).sqrt();
        if distance == 0.0 {
            return std::f32::INFINITY;
        } else {
            return distance;
        }
    }
}
