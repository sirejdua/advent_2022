use std::fs;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
use std::cmp;

/*
 *
 * FileSystem has interior mutability to modify its children, and its size
 * */

struct FileSystem {
    name: String,
    parent: Weak<FileSystem>,
    children: RefCell<Vec<Rc<FileSystem>>>,
    size: RefCell<u32>,
    is_dir: bool,
}

impl From<FileSystem> for Option<Box<FileSystem>> {
    fn from(fs: FileSystem) -> Self {
        Some(Box::new(fs))
    }
}

impl FileSystem {
    fn new_root() -> Self {
        FileSystem{
            name: "/".to_string(),
            parent: Weak::new(),
            children: RefCell::new(vec![]),
            size: Into::into(0),
            is_dir: true}
    }

    fn new_file(p: Weak<FileSystem>, name: String, size: u32) -> () {
        p.upgrade().unwrap().children.borrow_mut().push(Rc::new(FileSystem{
            name,
            parent: p,
            children: RefCell::new(vec![]),
            size: Into::into(size),
            is_dir: false}));
    }

    fn new_dir(p: Weak<FileSystem>, name: String) -> () {
        p.upgrade().unwrap().children.borrow_mut().push(Rc::new(FileSystem{
            name,
            parent: p,
            children: RefCell::new(vec![]),
            size: Into::into(0),
            is_dir: true}));
    }

    fn cd(&self, dir: &str) -> Option<Weak<FileSystem>> {
        if dir == ".." {
            return Some(Weak::clone(&self.parent));
        }

        
        for child in self.children.borrow().iter() {
            if dir == child.name {
                return Some(Rc::downgrade(&child));
            }
        }
        return None;
    }

    fn populate_dir_sizes(&self) -> () {
        if !self.is_dir {
            return ();
        }
        self.size.replace(0);

        for child in self.children.borrow().iter() {
            child.populate_dir_sizes();
        }
        
        for child in self.children.borrow().iter() {
            self.size.replace_with(|&mut old| old + *child.size.borrow());
        }

    }

    fn dir_names_satisfy_property(&self, result: &mut Vec<String>, p: &dyn Fn(&FileSystem) -> bool) {

        if p(self)
        {
            result.push(self.name.to_string());
        }

        for child in self.children.borrow().iter() {
            child.dir_names_satisfy_property(result, p);
        }
    }

    fn sum_size_if_property(&self, p: &dyn Fn(&FileSystem) -> bool) -> u32{
        let mut total_size: u32 = 0;

        if p(self)
        {
            total_size += *self.size.borrow();
            
        }

        for child in self.children.borrow().iter() {
            total_size += child.sum_size_if_property(p);
        }
        total_size
    }

    fn min_size_dir_satisfy_property(&self, p: &dyn Fn(&FileSystem) -> bool) -> u32 {
        let mut min_size: u32 = u32::MAX;

        if p(self)
        {
            min_size = cmp::min(min_size, *self.size.borrow());
        }

        for child in self.children.borrow().iter() {
            min_size = cmp::min(min_size, child.min_size_dir_satisfy_property(p));
        }

        min_size
    }

}

fn parse_ls(contents: Vec<&str>, cwd: Weak<FileSystem>) -> () {
    println!("in ls");
    println!("{:?}", contents);
    for listing in contents.iter() {
        let args: Vec<&str> = listing.split(" ").collect();
        match args[0] {
            "dir" => FileSystem::new_dir(cwd.clone(), args[1].to_string()),
            _     => FileSystem::new_file(cwd.clone(), args[1].to_string(), str::parse::<u32>(args[0]).unwrap())
        }
    }
}

fn parse_cd(dir: &str, cwd: Weak<FileSystem>) -> Option<Weak<FileSystem>> {
    println!("in cd");
    println!("{:?}", dir);

    return cwd.upgrade().unwrap().cd(dir);
}

fn main() {
    let file_path = "data.txt";

    println!("In file {}", file_path);
    let contents: String = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    
    let command_seq: Vec<&str> = contents.split("$").skip(1).collect();

    let root: Rc<FileSystem> = Rc::new(FileSystem::new_root());

    let mut cwd: Weak<FileSystem> = Rc::downgrade(&root);

    let threshold: u32 = 100000;

    for command in command_seq.iter().skip(1) {
        let lines: Vec<&str> = command.lines().collect();
        let line0: Vec<&str> = lines[0].split_whitespace().collect();
        println!("Command:");
        println!("{:?}", line0);
        match line0[0] {
            "ls" => parse_ls(lines[1..].to_vec(), Weak::clone(&cwd)),
            "cd" => cwd = parse_cd(line0[1], Weak::clone(&cwd)).expect("Should have found the directory"),
            _    => panic!("[ERROR]: {} is not in [cd, ls]", line0[0])
        };
    }

    root.populate_dir_sizes();
    let mut results: Vec<String> = vec![];
    root.dir_names_satisfy_property(&mut results, &|fs: &FileSystem| fs.is_dir && *fs.size.borrow() <= threshold);
    println!("Resulting dirs satisfy property");
    println!("{:?}", results);
    let result_sum: u32 = root.sum_size_if_property(&|fs: &FileSystem| fs.is_dir && *fs.size.borrow() <= threshold);
    println!("Sum of dir sizes satisfy property");
    println!("{:?}", result_sum);

    // part 2 : find smallest dir that will free up enough space
    let current_space: u32 =  *root.size.borrow();
    // total space: 70_000_000
    // free space: 70_000_000 - current_space
    // we want free space to be bigger than 30_000_000
    // find all dirs whose size is bigger than current_space - 40_000_000
    // find min of those
    println!("Min size directory to remove");
    let min_dir_size: u32 = root.min_size_dir_satisfy_property(&|fs: &FileSystem| fs.is_dir && *fs.size.borrow() >= current_space - 40_000_000);
    println!("{:?}", min_dir_size);
}
