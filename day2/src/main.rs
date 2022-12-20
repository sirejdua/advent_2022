// use std::env;
use std::{fs};

fn decode_move(opps: i32, trick: &str) -> i32 {
    match trick{
        "X" => (opps + 2) % 3,
        "Y" => opps,
        "Z" => (opps + 1) % 3,
        _   => 4
    }
}

fn translate_opps(opps: &str) -> i32 {
    match opps{
        "A"=> 0,
        "B"=> 1,
        "C"=> 2,
        _  => 3,
    }
}

fn translate_mine(mine: &str) -> i32 {
    match mine{
        "X"=> 0,
        "Y"=> 1,
        "Z"=> 2,
        _  => 3,
    }
}

fn score_move(opps: i32, mine: i32) -> i32 {
    let mut score = 0;
    if opps == mine {
        score = 3;
    }
    else if mine == (opps + 1) % 3 {
        score = 6;
    }
    return score + mine + 1;
}

fn main() {
    let file_path = "data.txt";

    println!("In file {}", file_path);

    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut my_score = 0;
    for line in lines {
        let xyz: Vec<&str> = line.split(" ").collect();
        assert_eq!(xyz.len(), 2);
        let opps = translate_opps(xyz[0]);
        let mine = decode_move(opps, xyz[1]);
        assert_ne!(opps, 3);
        assert_ne!(mine, 3);
        my_score += score_move(opps, mine);
    }

    println!("I scored {my_score}");

}

