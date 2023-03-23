// use sorting::quicksort;
use sorting::bubblesort;
use rand::{distributions::Uniform, Rng};
use std::time::Instant;

fn main() {

// ------------------------------------------------------------

    // println!("Quick sort:");
    // let mut arr = arr.clone();
//
    // let start = Instant::now();
    // quicksort(&mut arr);
    // let duration = start.elapsed();
    //
    // println!("Duration = {} ns",duration.as_nanos() as f64);

// ------------------------------------------------------------
    
    const ITERATIONS: u32 = 50;

    println!("\nBubble sort:");
    let mut rng = rand::thread_rng();
    let range = Uniform::new(-100,100);
    let mut arr: Vec<i64> = (-50..50).map(|_| rng.sample(&range)).collect();
    
    let mut times: Vec<f64> = Vec::new();
    for _ in 0..ITERATIONS{
        let start = Instant::now();
        bubblesort(&mut arr);
        let duration = start.elapsed();
        times.push(duration.as_micros() as f64);
    }
    
    let mut avg_us: f64 = 0.0;
    for i in 0..times.len() {
        avg_us += times[i];
    }
    avg_us = avg_us / times.len() as f64;

    println!("Average duration = {} us", avg_us);

// ------------------------------------------------------------

}

