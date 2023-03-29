use std::{fs, collections::{VecDeque, HashSet}};

fn all_different(ring: &VecDeque<char>) -> bool {
    let mut set: HashSet<char> = HashSet::new();
    for c in ring {
        set.insert(*c);
    }
    set.len() == ring.len()
}

fn main() {
    let file_path = "data_example.txt";

    println!("In file {}", file_path);
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    
    let tree_heights: Vec<u32> = contents.chars().filter(|&x| x != '\n').map(|c| c.to_digit(10).unwrap()).collect();
    let n: usize = (tree_heights.len() as f64).sqrt() as usize;


    let mut tree_set: Vec<Vec<bool>> = Vec::new();

    // left to right
    for i in 0..n
    {
        let left_to_right_row = tree_heights.iter().skip(i*n).take(n);
        let top_to_bottom_col = tree_heights.iter().skip(i).step_by(n);
        let right_to_left_row = tree_heights.iter().skip(i*n).take(n).rev();
        let bottom_to_top_col = tree_heights.iter().skip(i).step_by(n).rev();
    }

    println!("{:?}", tree_heights);

}
