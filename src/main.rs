<<<<<<< HEAD
mod aco_mem;
mod aco_memmap;
mod aco_no_mem;
extern crate fnv;
extern crate memmap;
use byteorder::{ByteOrder, LittleEndian};
use memmap::MmapOptions;
use std::f32::INFINITY;
use fnv::FnvHashMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::mem::transmute;
use std::{mem, slice};

//fn main() {
//println!("Hello, world!");
//let f = File::open("/home/datascience/Documents/aco_rust/cities.csv").unwrap();
//let file = BufReader::new(&f);
//let mut coordinates: Vec<aco_memmap::Coordinate> = Vec::new();
//let mut length = 0;
//for line in file.lines() {
//length += 1;
//let line = line.unwrap();
//let line = line.trim();
//
//let line: Vec<&str> = line.split(",").collect();
////println!("{:?}", line);
//coordinates.push(aco_memmap::Coordinate {
//x: line[1].parse().unwrap(),
//y: line[2].parse().unwrap(),
//});
//}
//
//println!("finished coordinate file");

//{
//let mut f = File::create("/data1/temp").expect("Unable to create file");
//let mut file = BufWriter::new(&f);
//
//let mut count = 0;
//for i in &coordinates {
//println!("{:?}", count);
//count += 1;
//for j in &coordinates {
//let number = aco_memmap::euclidean_distance(i, j);
//let raw_bytes: [u8; mem::size_of::<f32>()] = unsafe { std::mem::transmute(number) };
//file.write(&raw_bytes).unwrap();
//}
//}
//}

//let mut f = File::open("/data1/temp").unwrap();
//println!("read");
//let mmap = unsafe { MmapOptions::new().map(&f).unwrap() };
//
//let mut antcolony = aco_memmap::AntColony {
//no_ants: 4,
//no_best_paths: 100,
//no_iterations: 200,
//decay: None,
//alpha: 1.0,
//beta: 1.0,
//distances: mmap,
//length: length,
//initial_tour: None,
//default_pheromone: 0.0,
//nodes: Vec::new(),
//pheromones: HashMap::new(),
//};
//println!("{:?}", antcolony.run());
//}

fn main0() {
    println!("Hello, world!");
    let f = File::open("/home/datascience/Documents/rust_benchmarks/att48_d.txt").unwrap();
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
                    if dis == 0.0 {
                        std::f64::INFINITY
                    } else {
                        dis
                    }
                })
                .collect(),
        );
    }

    {
        let mut f = File::create("temp").expect("Unable to create file");
        let mut file = BufWriter::new(&f);

        let mut count = 0;
        for i in &distance {
            //println!();
            //println!("{:?}", i);
            println!("{:?}", count);
            count += 1;
            for j in i {
                //print!("{} ",j);
                let raw_bytes: [u8; mem::size_of::<f64>()] = unsafe { std::mem::transmute(*j) };
                file.write(&raw_bytes).unwrap();
            }
        }
    }

    let mut f = File::open("temp").unwrap();
    println!("read");
    let mmap = unsafe { MmapOptions::new().map(&f).unwrap() };

    println!("distance length {}", distance.len());

    let mut antcolony = aco_memmap::AntColony {
        no_ants: 200,
        no_best_paths: 100,
        no_iterations: 200,
        decay: None,
        alpha: 1.0,
        beta: 1.0,
        distances: mmap,
        length: distance.len() as i64,
        initial_tour: None,
        default_pheromone: 0.0,
        nodes: Vec::new(),
        pheromones: HashMap::new(),
    };

    //for i in 0..distance.len(){
    //println!("distance ---> {:?}", distance[i]);
    //println!("file ---> {:?}", antcolony.get_distance(i as i64));
    //}
    println!("inside fun");
    println!("{:?}", antcolony.run());
    //println!("{:?}", distance);
}
fn main4() {
    println!("Hello, world!");
    let f = File::open("/home/datascience/Documents/rust_benchmarks/att48_d.txt").unwrap();
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
                    if dis == 0.0 {
                        std::f64::INFINITY
                    } else {
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

fn main1() {
    println!("Hello, world!");
    let f = File::open("/home/datascience/Documents/rust_benchmarks/att48_d.txt").unwrap();
    let file = BufReader::new(&f);
    let mut distance: Vec<Vec<f32>> = Vec::new();
    for line in file.lines() {
        let line = line.unwrap();
        let line = line.trim();
        let line: Vec<&str> = line.split("      ").collect();
        distance.push(
            line.iter()
                .map(|x| {
                    let dis = x.trim().parse().unwrap();
                    if dis == 0.0 {
                        INFINITY
                    } else {
                        dis
                    }
                })
                .collect(),
        );
    }
    println!("1st element {:?} {}", distance[0], distance[0].len());

    let mut f = File::create("temp").expect("Unable to create file");
    let mut file = BufWriter::new(&f);
    for i in &distance {
        for j in i {
            let raw_bytes: [u8; mem::size_of::<f32>()] = unsafe { std::mem::transmute(*j) };
            file.write(&raw_bytes).unwrap();
        }
    }

    let f = File::open("temp").unwrap();
    println!("read");
    let mmap = unsafe { MmapOptions::new().map(&f).unwrap() };
    for i in 0..distance[0].len() {
        let number = LittleEndian::read_f32(
            &mmap[i * mem::size_of::<f32>()..(i + 1) * mem::size_of::<f32>()],
        );
        //println!("bytes {:?}", &mmap[i*8..(i+1)*8]);
        print!("{:?},", number);
    }
    println!("");
    //println!("{:?}", distance);
}

fn main() {
    println!("Hello, world!");
    let f = File::open("/home/datascience/Documents/aco_rust/cities.csv").unwrap();
    //let f = File::open("../../cities.csv").unwrap();
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

    let mut antcolony = aco_no_mem::AntColony {
        no_ants: 10,
        no_best_paths: 100,
        no_iterations: 100,
        decay: None,
        alpha: 1.0,
        beta: 1.0,
        distances: distance,
        initial_tour: None,
        default_pheromone: 0.0,
        nodes: Vec::new(),
        pheromones: FnvHashMap::default(),
        //pheromones: HashMap::default(),
    };
    println!("{:?}", antcolony.run());
=======
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

>>>>>>> 4f06b9520be8810492ea63d71e307b7d7b24ea31
}
