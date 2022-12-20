// use std::env;
use std::{fs, collections::BinaryHeap};

fn main() {
    let file_path = "data.txt";

    println!("In file {}", file_path);

    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let num_array = contents.lines();
    let mut sum = 0;
    let mut heap = BinaryHeap::new();

    /*
    use partition_point to find the value and insert
    let insert_idx = largest.partition_point(|x| x < sum);
    move 0..insert_idx to the left
    insert at insert_idx

    ... or use a min heap
    https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html
    
    */
    heap.push(0);
    for num in num_array {
        if num == "" {
            if heap.len() == 3 {
                if sum < *heap.peek().unwrap() {
                    heap.pop();
                    heap.push(sum);
                }
            } else {
                heap.push(sum);
            }
            sum = 0;
        } else {
            sum -= num.parse::<i32>().unwrap();
        }
    }
    let mut hungriest = 0;
    while heap.peek() != None
    {
        hungriest -= *heap.peek().unwrap();
        heap.pop();
    }
    println!("Hungriest elves total:\n{hungriest}");
}
