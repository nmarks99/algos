
pub fn binary_search<T>(sorted_arr: &[T], target_val: T) -> Option<usize>
where
    T: PartialOrd + Copy,
{ 
    
    // number of elements in the array
    let n: i32 = sorted_arr.len().try_into().unwrap();
    
    // Start with the left index at the first element
    // rigth index at the last element
    let mut left: i32 = 0;
    let mut right: i32 = n - 1;
    
    // if left > right, failed to find the target value, return None
    while left <= right {

        let m = ((left + right) as f64/ 2.0).floor() as usize;

        if sorted_arr[m] < target_val {
            left = m as i32 + 1;
        }
        else if sorted_arr[m] > target_val {
            right = m as i32 - 1; 
        }
        else {
            return Some(m);
        }
    }

    None

}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_binary_search_found() {
        let v: Vec<i8> = vec![1,2,3,4,5];
        let ind = binary_search(&v, 3);
        assert_eq!(ind.unwrap(), 2);

        let a: [f32; 5] = [1.5,2.2,3.2,4.0,5.0];
        let ind = binary_search(&a, 3.2);
        assert_eq!(ind.unwrap(), 2);
    }

    #[test]
    fn test_binary_search_not_found() {
        let v: Vec<i8> = vec![1,2,3,4,5];
        let ind = binary_search(&v, 7);
        assert!(ind.is_none());
    }


    #[test]
    fn test_binary_search_random() {
        
        use rand::{distributions::Uniform, Rng};

        // Fills a vector with n random values between min_rand and max_rand
        fn gen_random_vec(n: usize, min_rand: i64, max_rand: i64) -> Vec<i64> {
            let mut rng = rand::thread_rng();
            let range = Uniform::new(min_rand,max_rand+1);
            let n: i64 = (n/2) as i64;
            let arr: Vec<i64> = (-n..n).map(|_| rng.sample(&range)).collect();
            arr
        }
        
        for _ in 0..100 {
            let mut v = gen_random_vec(10, -10, 10);
            v.sort();
            let mut rng = rand::thread_rng();
            let target_val = v[rng.gen_range(0..10)];
            let ind = binary_search(&v, target_val);
            assert_eq!(target_val, v[ind.unwrap()]);
            // assert_eq!(ind.unwrap(), v.iter().position(|&r| r == target_val).unwrap());
        }
    }




}
