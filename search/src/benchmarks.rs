use search::binary_search;

fn main() { 
   
    let v = vec![1.0,3.0,5.0,9.0];
    let target_value = 3.0;
    let ind = binary_search(&v, target_value);
    println!("Target value {} located at index {}", target_value, ind.unwrap());

}
