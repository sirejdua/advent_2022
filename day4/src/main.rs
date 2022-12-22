use std::{fs};

fn contains(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    x2 >= x1 && y2 <= y1
}

fn overlaps(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    x2 >= x1 && x2 <= y1 || y2 >= x1 && y2 <= y1
}
fn main() {
    let file_path = "data.txt";

    println!("In file {}", file_path);

    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut sum = 0;
    for line in lines {
        let ranges: Vec<&str> = line.split(',').collect();
        let elf1: Vec<&str> = ranges[0].split('-').collect();
        let elf2: Vec<&str> = ranges[1].split('-').collect();
        let x1 = elf1[0].parse::<i32>().unwrap();

        let y1 = elf1[1].parse::<i32>().unwrap();

        let x2 = elf2[0].parse::<i32>().unwrap();
        let y2 = elf2[1].parse::<i32>().unwrap();
        if overlaps(x1, y1, x2, y2) || overlaps(x2, y2, x1, y1) {
            sum += 1;
        }
    }
    println!("Number of lazy elves {}", sum);

}
