pub fn binary_search<T>(sorted_arr: &[T], target_val: T) -> Option<usize>
where
    T: PartialOrd + Copy + Default,
{ 
    
    // number of elements in the array
    let n: i32 = sorted_arr.len().try_into().unwrap();
    
    // Start with the left index at the first element
    // rigth index at the last element
    let mut left: i32 = 0;
    let mut right: i32 = n - 1;
    
    // if left > right, failed to find the target value, return None
    while left < right {

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
        assert_eq!(ind, None);
    }
}
