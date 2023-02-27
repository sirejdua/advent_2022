use std::fs;

struct FileSystem {
    name: String,
    parent: Option<Box<FileSystem>>,
    children: Vec<Option<Box<FileSystem>>>,
    size: u32,
}

impl From<FileSystem> for Option<Box<FileSystem>> {
    fn from(fs: FileSystem) -> Self {
        Some(Box::new(fs))
    }
}

impl FileSystem {
    fn new_file(&mut self, name: String, size: u32) -> Self {
        FileSystem{
            name,
            parent: *self.into(),
            children: vec![],
            size}
    }
}

fn call_cd(dir: &str, contents: Vec<&str>) -> () {
    println!("in cd");
}

fn call_ls() -> () {
    println!("in ls");
}

fn main() {
    let file_path = "data_example.txt";

    println!("In file {}", file_path);
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    
    let command_seq: Vec<&str> = contents.split("$").skip(1).collect();

    for command in command_seq {
        let lines: Vec<&str> = command.lines().collect();
        let line0: Vec<&str> = lines[0].split_whitespace().collect();
        match line0[0] {
            "cd" => call_cd(line0[1], lines[1..].to_vec()),
            "ls" => call_ls(),
            _    => panic!("[ERROR]: {} is not in [cd, ls]", line0[0])
        };
        println!("{:?}", line0);

    }
}
