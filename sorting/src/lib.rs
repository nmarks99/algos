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

pub fn quicksort(v: &mut Vec<i64>, left: usize, right:usize) {
    
    if left < right {
        let partition_index: usize = partition(v, left, right); 
        if partition_index == 0 {
            quicksort(v, left, v.len()-1);
        }
        else
        {
            quicksort(v, left, partition_index - 1);
        }
        quicksort(v, partition_index + 1, right);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_swap() {
        let mut v = vec![1,2,3,4,5];
        swap(&mut v, 0, 4);
        let v_correct = vec![5,2,3,4,1];
        assert_eq!(v, v_correct);
    }

    #[test]
    fn test_quicksort() {
        let mut v1 = vec![5,4,3,2,1];
        let len1 = v1.len();
        let v1_correct = vec![1,2,3,4,5];
        quicksort(&mut v1, 0, len1 - 1);
        assert_eq!(v1, v1_correct);

        let mut v2 = vec![5,4,-3,2,1];
        let len2 = v2.len();
        let v2_correct = vec![-3,1,2,4,5];
        quicksort(&mut v2, 0, len2 - 1);
        assert_eq!(v2,v2_correct);

        let mut v3 = vec![3, 4, -2, -14, 5, 2, 5, 3, -53];
        let len3 = v3.len();
        let v3_correct = vec![-53, -14, -2, 2, 3, 3, 4, 5, 5];
        quicksort(&mut v3, 0, len3 - 1);
        assert_eq!(v3,v3_correct);
    }
}
