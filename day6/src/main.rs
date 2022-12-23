use std::{fs, collections::{VecDeque, HashSet}};

fn all_different(ring: &VecDeque<char>) -> bool {
    let mut set: HashSet<char> = HashSet::new();
    for c in ring {
        set.insert(*c);
    }
    set.len() == ring.len()
}

fn main() {
    let file_path = "data.txt";

    println!("In file {}", file_path);
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let mut ring: VecDeque<char> = VecDeque::new();
    let mut i = 0;
    for c in contents.chars() {
        if ring.len() < 14 {
            ring.push_back(c);
        } else {
            if all_different(&ring) {
                break;
            }
            ring.pop_front();
            ring.push_back(c);
        }
        i += 1;
    }
    println!("{:?}", ring);
    println!("{:?}", i);

}
