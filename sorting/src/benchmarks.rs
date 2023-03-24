use sorting::quicksort;
use sorting::bubblesort;
use rand::{distributions::Uniform, Rng};
use std::time::Instant;

const ITERATIONS: u64 = 100;
const MIN_RAND: i64 = -100;
const MAX_RAND: i64 = 100;
const ARR_SIZE: usize = 1000;

// Fills a vector with n random values between min_rand and max_rand
fn gen_random_vec(n: usize, min_rand: i64, max_rand: i64) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(min_rand,max_rand);
    let n: i64 = (n/2) as i64;
    let arr: Vec<i64> = (-n..n).map(|_| rng.sample(&range)).collect();
    arr
}


// Computes the average of the elements in a Vec<f64>
fn vec_avg(vec: &Vec<f64>) -> f64 {
    let mut avg: f64 = 0.0;
    for i in 0..vec.len() {
        avg += vec[i]; 
    }
    avg / vec.len() as f64
}

// Times how long the function f takes to sort an 
// array generated from gen_random_vec
fn timeit(f: fn(&mut Vec<i64>)) -> f64 {

    let mut times: Vec<f64> = Vec::new();
    for _ in 0..ITERATIONS {
        let mut arr = gen_random_vec(ARR_SIZE, MIN_RAND, MAX_RAND);
        let start = Instant::now();
        f(&mut arr);
        let duration = start.elapsed();
        times.push(duration.as_micros() as f64);
    }
    let avg_us = vec_avg(&times);
    avg_us

}

fn main() {

    println!("Benchmarking");
    println!("Iterations: {}", ITERATIONS);
    println!("Elements: {}", ARR_SIZE);
    println!("------------------");

// ------------------------------------------------------------
    
    println!("Quick sort:");
    let avg_us = timeit(quicksort);
    println!("Average = {} us",avg_us);

// ------------------------------------------------------------
    
    println!("\nBubble sort:");
    let avg_us = timeit(bubblesort);
    println!("Average = {} us", avg_us);

// ------------------------------------------------------------

}

