use sorting::quicksort;
use sorting::bubblesort;
use rand::{distributions::Uniform, Rng};
use std::time::Instant;

const ITERATIONS: usize = 50;
const MIN_RAND: i64 = -100;
const MAX_RAND: i64 = 100;
const ARR_SIZE: i64 = 1000;

fn gen_random_vec(arr_size: i64, min_rand: i64, max_rand: i64) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(min_rand,max_rand);
    let arr: Vec<i64> = (-arr_size..arr_size).map(|_| rng.sample(&range)).collect();
    arr
}

fn vec_avg(vec: &Vec<f64>) -> f64 {
    let mut avg: f64 = 0.0;
    for i in 0..vec.len() {
        avg += vec[i]; 
    }
    avg / vec.len() as f64
}

fn main() {
    
// ------------------------------------------------------------
    
    println!("Benchmarking for {} iterations\n", ITERATIONS);

    println!("Quick sort:");
    let mut arr = gen_random_vec(ARR_SIZE, MIN_RAND, MAX_RAND);

    let mut times: Vec<f64> = Vec::new();
    for _ in 0..ITERATIONS {
        let start = Instant::now();
        quicksort(&mut arr);
        let duration = start.elapsed();
        times.push(duration.as_millis() as f64);
    }

    let avg_us = vec_avg(&times);
    
    println!("Average = {} ms",avg_us);

// ------------------------------------------------------------
    
    println!("\nBubble sort:");
    let mut arr = gen_random_vec(ARR_SIZE, MIN_RAND, MAX_RAND);

    let mut times: Vec<f64> = Vec::new();
    for _ in 0..ITERATIONS{
        let start = Instant::now();
        bubblesort(&mut arr);
        let duration = start.elapsed();
        times.push(duration.as_millis() as f64);
    }
    
    let avg_us = vec_avg(&times);

    println!("Average = {} ms", avg_us);

// ------------------------------------------------------------

}

