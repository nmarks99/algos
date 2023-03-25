use search::binary_search;
use rand::{distributions::Uniform, Rng};
use std::time::Instant;


// Fills a vector with n random values between min_rand and max_rand
fn gen_random_vec(n: usize, min_rand: i64, max_rand: i64) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(min_rand,max_rand+1);
    let n: i64 = (n/2) as i64;
    let arr: Vec<i64> = (-n..n).map(|_| rng.sample(&range)).collect();
    arr
}

// computes the average value of a Vec<f64>
// fn vec_avg(vec: &Vec<f64>) -> f64 {
    // let mut avg: f64 = 0.0;
    // for i in 0..vec.len() {
        // avg += vec[i];
    // }
    // avg / vec.len() as f64
// }

fn main() { 
    
    const MIN_RAND: i64 = -1000;
    const MAX_RAND: i64 = 1000;
    const ARR_SIZE: usize = 100000;
    
    // Get a random vector and sort it
    let mut v = gen_random_vec(ARR_SIZE, MIN_RAND, MAX_RAND);
    v.sort();

    // Get a random target value
    let mut rng = rand::thread_rng();
    let target_value = v[rng.gen_range(0..ARR_SIZE)];

    // Run binary search and time it
    let start = Instant::now();
    let _ind = binary_search(&v, target_value);
    let elapsed = start.elapsed();

    println!("Elapsed time = {}", elapsed.as_nanos());
}
