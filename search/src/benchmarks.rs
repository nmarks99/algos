use search::binary_search;
use rand::{distributions::Uniform, Rng};
// use std::time::Instant;


// Fills a vector with n random values between min_rand and max_rand
fn gen_random_vec(n: usize, min_rand: i64, max_rand: i64) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(min_rand,max_rand+1);
    let n: i64 = (n/2) as i64;
    let arr: Vec<i64> = (-n..n).map(|_| rng.sample(&range)).collect();
    arr
}

fn main() { 
    
    // Get a random vector and sort it
    const MIN_RAND: i64 = -100;
    const MAX_RAND: i64 = 100;
    const ARR_SIZE: usize = 10;

    let mut v = gen_random_vec(ARR_SIZE, MIN_RAND, MAX_RAND);
    v.sort();
    println!("{:?}",v);
    println!("{:?}",v.len());

    // Get a random target value
    let mut rng = rand::thread_rng();
    let rand_ind = rng.gen_range(0..ARR_SIZE);
    println!("rand_ind = {}", rand_ind);
    let target_value = v[rand_ind];

    let ind = binary_search(&v, target_value);

    if ind.is_none() {
        println!("Target value {} not found in vector", target_value);
    }
    else 
    {
        println!("target value {} at index {}", target_value, ind.unwrap());
    }

}
