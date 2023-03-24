#![allow(dead_code)]

fn swap(v: &mut Vec<i64>, i: usize, j: usize) {
    let vi = v[i];
    let vj = v[j];
    v[i] = vj;
    v[j] = vi;
}

fn partition(v: &mut Vec<i64>, left: i32, right:i32) -> i32 {
    
    let pivot = v[right as usize]; // let last element be pivot
    let mut i: i32 = left - 1;

    for j in left..right {
        if v[j as usize] < pivot {
            i += 1;
            swap(v,i.try_into().unwrap(),j.try_into().unwrap());
        }
    }
    swap(v, (i+1).try_into().unwrap(), right.try_into().unwrap());
    
    return (i + 1).try_into().unwrap();
}


fn _quicksort(v: &mut Vec<i64>, left: i32, right:i32) {
    
    if left < right {
        let partition_index: i32 = partition(v, left, right); 
        _quicksort(v, left, partition_index - 1);
        _quicksort(v, partition_index + 1, right);
    }
}

pub fn quicksort(v: &mut Vec<i64>) {
    _quicksort(v, 0, (v.len() - 1) as i32);
}

pub fn bubblesort(v: &mut Vec<i64>) {
    let n = v.len();
    for i in 0..n {
        for j in 0..(n-i-1) {
            if v[j] > v[j+1] {
                swap(v, j, j+1);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_swap() {
        let mut v = vec![1,2,3,4,5];
        swap(&mut v, 0, 4);
        let v_correct = vec![5,2,3,4,1];
        assert_eq!(v, v_correct);
    }

    #[test]
    fn test_quicksort() {
        let mut v1 = vec![5,4,3,2,1];
        let v1_correct = vec![1,2,3,4,5];
        quicksort(&mut v1);
        assert_eq!(v1, v1_correct);

        let mut v2 = vec![5,4,-3,2,1];
        let v2_correct = vec![-3,1,2,4,5];
        quicksort(&mut v2);
        assert_eq!(v2,v2_correct);

        let mut v3 = vec![3, 4, -2, -14, 5, 2, 5, 3, -53];
        let v3_correct = vec![-53, -14, -2, 2, 3, 3, 4, 5, 5];
        quicksort(&mut v3);
        assert_eq!(v3,v3_correct);

        let mut v4 = vec![3, 4, -2, -14];
        let v4_correct = vec![-14,-2,3,4];
        quicksort(&mut v4);
        assert_eq!(v4,v4_correct);
    }

    #[test]
    fn test_bubblesort() {
        let mut v = vec![5,4,3,2,1];
        let v_correct = vec![1,2,3,4,5];
        bubblesort(&mut v);
        assert_eq!(v, v_correct);
    }
}
