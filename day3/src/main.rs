use std::{fs, collections::HashSet};

fn elf_to_set(elf: &[u8]) -> HashSet<u8> {
    let mut set = HashSet::new();
    for c in elf {
        set.insert(*c);
    }
    set
}

fn score_group(elf1: &str, elf2: &str, elf3: &str) -> i32 {
    let set1 = elf_to_set(elf1.as_bytes());
    let set2 = elf_to_set(elf2.as_bytes());
    let set3 = elf_to_set(elf3.as_bytes());

    let intersection1 = &set1 & &set2;
    let intersection2 = &intersection1 & &set3;

    let mut z = 0;
    for x in intersection2 {
        println!("x: {}", x as char);
        z = score(x);
        println!("z: {}", z);
    }
    z
}


fn score(c: u8) -> i32 {
    if c >= 97 {
        return i32::from(c) - 97 + 1;
    } else {
        return 26 + i32::from(c) - 65 + 1;
    }
}

fn compute_score(rucksack: &str) -> i32 {
    let n = rucksack.len();
    assert!(n % 2 == 0);
    let m = n / 2;
    let array = rucksack.as_bytes();
    let left = &array[0 .. m];
    let right = &array[m .. n];
    // make a left hashset
    let mut left_set = HashSet::new();
    for l in left {
        left_set.insert(*l);
    }
    for r in right {
        if left_set.contains(&*r) {
            println!("Found {r} aka {}", *r as char);
            let s = score(*r);
            println!("Score {s}");
            return s;
        }
    }
    0
}

fn main() {
    let file_path = "data.txt";

    println!("In file {}", file_path);

    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    let mut sum = 0;

    for i in 0..lines.len()/3 {
        let j = i * 3;
        let s = score_group(lines[j], lines[j+1], lines[j+2]);
        println!("score: {}", s);
        sum += s;
    }
    println!("Rucksack total {}", sum);

}
