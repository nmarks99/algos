use sorting::quicksort;

fn main() {
    
    let mut arr = vec![3, 4, -2, -14, 5, 2, 5, 3, -53];
    println!("Unsorted = {:?}", arr);
    quicksort(&mut arr);
    println!("Sorted = {:?}", arr);
}
