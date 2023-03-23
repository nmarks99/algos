#![allow(dead_code)]

fn swap(v: &mut Vec<i64>, i: usize, j: usize) {
    let vi = v[i];
    let vj = v[j];
    v[i] = vj;
    v[j] = vi;
}

fn partition(v: &mut Vec<i64>, left: usize, right:usize) -> usize {

    let mut i = left;
    let mut j = right - 1;
    let pivot = v[right]; // let last element be pivot
    
    // i moves right, j moves left until i and j cross
    while i < j {
        while i < right && v[i] < pivot {
            i += 1;
        }

        while j > left && v[j] >= pivot {
            j -= 1;
        }

        if i < j {
            swap(v, i, j);
        }
    }
    
    if v[i] > pivot {
        swap(v, i, right);
    }
    
    return i;
}


fn quicksort_recursive(v: &mut Vec<i64>, left: usize, right:usize) {
    
    if left < right {
        let partition_index: usize = partition(v, left, right); 
        let right_tmp: usize;

        if partition_index == 0 {
            right_tmp = v.len() - 2;
        }
        else {
            right_tmp = partition_index - 1;
        }

        let left_tmp = partition_index + 1;
        quicksort_recursive(v, left, right_tmp);
        quicksort_recursive(v, left_tmp, right);
    }
}

pub fn quicksort(v: &mut Vec<i64>) {
    quicksort_recursive(v, 0, v.len() - 1);
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
