extern crate permutation;
extern crate rand;
extern crate rayon;
use byteorder::{ByteOrder, LittleEndian};
use memmap::Mmap;
use memmap::MmapOptions;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::f32::INFINITY;
use std::f32::MAX;
use std::mem::transmute;
use std::{mem, slice};

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

#[derive(Debug)]
pub struct AntColony {
    pub no_ants: i64,
    pub no_best_paths: i64,
    pub no_iterations: i64,
    pub decay: Option<i64>,
    pub alpha: f32,
    pub beta: f32,
    pub distances: Mmap,
    pub length: i64,
    pub nodes: Vec<i64>,
    pub initial_tour: Option<Vec<i64>>,
    //pub pheromones: Vec<Vec<f32>>,
    pub pheromones: HashMap<i64, HashMap<i64, f32>>,
    pub default_pheromone: f32,
}

impl AntColony {
    pub fn get_distance(&self, start_pos: i64) -> Vec<f32> {
        let mut temp: Vec<f32> = Vec::new();
        let start_pos = start_pos * self.length;
        for i in start_pos..start_pos + self.length {
            let number = LittleEndian::read_f32(
                &self.distances
                    [i as usize * mem::size_of::<f32>()..(i + 1) as usize * mem::size_of::<f32>()],
            );
            temp.push(number);
        }
        return temp;
    }

    pub fn get_distance_value(&self, start_pos_x: i64, start_pos_y: i64) -> f32 {
        let start_pos = (start_pos_x * self.length) + start_pos_y;
            let number = LittleEndian::read_f32(
                &self.distances[start_pos as usize * mem::size_of::<f32>()..(start_pos+1) as usize * mem::size_of::<f32>()]);
            return number;
    }
}

#[derive(Default, Debug, Clone)]
pub struct UnitPath {
    pub start: i64,
    pub end: i64,
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
        for i in 0..self.length {
            self.nodes.push(i as i64);
            //println!("distance {:?} {:?}", i, self.get_distance(i));
        }
        //self.pheromones = Vec::new();
        self.default_pheromone = 1.0 / (self.length as f32);

        //for row in 0..self.length {
        //let mut row_values: Vec<f32> = Vec::new();
        //for _value in 0..self.length {
        //row_values.push(self.default_pheromone);
        //}
        //self.pheromones.push(row_values);
        //}
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
            .par_iter()
            .map(|x| {
                println!("ant {}", x);
                self.gen_path(0)
            })
            .map(|x| TourPath {
                distance: self.path_distance(&x),
                path: x,
            })
            .collect();
    }

    fn gen_path(&self, start: i64) -> Vec<UnitPath> {
        println!("inside genpath");
        let mut antpath: Vec<UnitPath> = Vec::new();
        let mut visited_nodes: HashSet<i64> = HashSet::new();
        visited_nodes.insert(start);
        let mut prev = start;
        for i in 0..(self.length - 1) {
            if i % 100 == 0 {
                println!("inside distance in genpath {}", i);
            }
            //let pheromone_prev: Vec<f32> = self.pheromones[prev as usize].clone();
            let pheromone_prev: Vec<f32> = match self.pheromones.get(&prev) {
                Some(val) => (0..self.length)
                    .into_iter()
                    .map(|j| {
                        match val.get(&(j as i64)) {
                            Some(val1) => val1,
                            None => &self.default_pheromone,
                        }
                        .clone()
                    })
                    .collect(),
                None => (0..self.length)
                    .into_iter()
                    .map(|_j| (self.default_pheromone))
                    .collect(),
            };

            //println!("pheromones ----------> {:?}", &pheromone_prev);
            let next = self.pick_next_move(
                pheromone_prev,
                //&self.distances[prev as usize],
                &self.get_distance(prev),
                &visited_nodes,
            );
            //println!("next------>{:?}", next);
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
        mut pheromone: Vec<f32>,
        distances: &Vec<f32>,
        visited: &HashSet<i64>,
    ) -> i64 {
        //println!("visited------> {:?}", visited);
        //println!("distances------> {:?}", distances);
        for i in visited {
            pheromone[*i as usize] = 0.0;
        }
        //println!("pheromone------> {:?}", pheromone);

        let mut row: Vec<f32> = Vec::new();

        for i in 0..pheromone.len() {
            //into is for converting between f32 and f32

            //let value = (pheromone[i]).powf(self.alpha) * (1.0 / distances[i]).powf(self.beta);
            //print!("{} {},", pheromone[i], distances[i]);
            let value = (pheromone[i]) * (1.0 / distances[i]);
            row.push(value);
        }
        //println!();
        //println!("row---------->{:?}", row);
        //let row_sum: f32 = row.iter().sum();

        //println!("row_sum---------->{:?}", row_sum);
        //for i in 0..row.len() {
            //row[i] = row[i] / row_sum;
        //}

        //println!("row---------->{:?}", row);
        let dist = WeightedIndex::new(&row).unwrap();
        let mut rng = thread_rng();
        self.nodes[dist.sample(&mut rng)]
    }

    fn path_distance(&self, iteration_path: &Vec<UnitPath>) -> f32 {
        let mut distance = 0.0;
        for edge in iteration_path {
            distance += self.get_distance_value(edge.start,edge.end);
        }
        return distance;
    }

    fn spread_pheromone(&mut self, iteration_paths: &Vec<TourPath>, no_best_path: i64) {
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
                let distance;
                {
                    distance = self.get_distance_value(edge.start,edge.end);
                }
                let inb = self.pheromones.entry(edge.start).or_insert(HashMap::new());
                //*inb.entry(edge.end).or_insert(self.default_pheromone) += 1.0/(&self.get_distance(edge.start)[edge.end as usize]);
                *inb.entry(edge.end).or_insert(self.default_pheromone) += 1.0 / distance;
            }
        }
    }
}
pub fn euclidean_distance(a: &Coordinate, b: &Coordinate) -> f32 {
    //let distance = ((a.x - b.x).powf(2.0) + (a.y - b.y).powf(2.0)).sqrt();
    let distance = ((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)).sqrt();
    if distance == 0.0 {
        return std::f32::INFINITY;
    } else {
        return distance;
    }
}
