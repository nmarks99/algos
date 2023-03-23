use sorting::quicksort;

fn main() {
    
    let mut arr = vec![5,4,3,2,1];
    let len = arr.len();
    println!("Unsorted = {:?}", arr);
    quicksort(&mut arr, 0, len-1);
    println!("Sorted = {:?}", arr);
}
