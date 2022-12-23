use std::{fs, str};

fn process_diagram(diagram: &Vec<Vec<u8>>, num_stacks: usize) -> Vec<Vec<u8>> {
    println!("{:?}", diagram);
    println!("{:?}", num_stacks);
    let mut stacks: Vec<Vec<u8>> = Vec::new();
    if diagram.len() == 0 {
        return stacks;
    }

    for _k in 0..num_stacks {
        stacks.push(Vec::new());
    }

    for i in 0..diagram.len() {
        let j = diagram.len() - i - 1;
        for k in 0..num_stacks {
            if diagram[j][k] != 32 {
                stacks[k].push(diagram[j][k]);
            }
        }
    }
    println!("stacks");
    println!("{:?}", stacks);
    stacks
}

fn main() {
    let file_path = "data.txt";

    println!("In file {}", file_path);

    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut phase1 = true;

    let mut diagram: Vec<Vec<u8>> = Vec::new();
    let mut stacks: Vec<Vec<u8>> = Vec::new();

    let mut num_stacks = 0;

    for line in lines {
        let v = line.as_bytes();
        println!("v.len()={}", v.len());
        println!("phase1={}", phase1);
        if phase1 {
            if v.len() == 0 {
                phase1 = false;
                stacks = process_diagram(&diagram, num_stacks);
                continue;
            }
            let mut next_line: Vec<u8> = Vec::new();
            num_stacks = v.len()/4 + 1;
            for i in 0 .. num_stacks {
                let c = v[4*i + 1];
                if (c as char).to_digit(10).is_some() {
                    continue;
                }
                next_line.push(c);
            }
            if next_line.len() != 0 {
                diagram.push(next_line);
            }
        } else {
            let tokens: Vec<usize> = line
                                    .split_whitespace()
                                    .map(|x: &str| x.parse::<usize>())
                                    .filter(|x| x.is_ok())
                                    .map(|x| x.unwrap())
                                    .collect();
            
            let num_to_move = tokens[0];
            let from        = tokens[1];
            let to          = tokens[2];

            let mut temp_stack: Vec<u8> = Vec::new();
            for _k in 0..num_to_move {
                let val = stacks[from-1].pop().unwrap();
                temp_stack.push(val);
            }
            for _k in 0..num_to_move {
                let val = temp_stack.pop().unwrap();
                stacks[to-1].push(val);
            }
        }
    }
    let mut f: Vec<u8> = Vec::new();
    for k in 0..num_stacks {
        let last_val = stacks[k].pop();
        if last_val.is_none() {
            continue;
        }
        f.push(last_val.unwrap());
    }
    let sl = f.as_slice();
    let output = str::from_utf8(&sl).unwrap();
    println!("Gift boxes left {}", output);

}
