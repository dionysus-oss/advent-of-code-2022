use common::read_lines;
use dir::DirContent;
use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::Peekable;
use std::path::{Path, PathBuf};
use std::slice::Iter;

fn main() {
    let shell_output: Vec<String> = read_lines("input.txt")
        .unwrap()
        .map(|line| line.unwrap())
        .collect();

    let mut dir_sizes = HashMap::new();
    let mut iter = shell_output.iter().peekable();
    let mut cwd = Path::new("/").to_path_buf();
    while iter.peek() != None {
        if let Some(new_cwd) = process_next_command(&mut iter, &mut dir_sizes, cwd.clone()) {
            cwd = new_cwd.as_path().to_owned();
        }
    }

    compute_child_sizes(&dir_sizes, "/".to_string());

    let total_for_small_dirs: i32 = dir_sizes
        .values()
        .map(|dir: &RefCell<DirContent>| dir.borrow().get_total_size().unwrap())
        .filter(|v| *v <= 100000)
        .sum();
    println!("Total for small directories {}", total_for_small_dirs);

    let fs_size = 70_000_000;
    let required_space = 30_000_000;

    let current_used = dir_sizes
        .get("/")
        .unwrap()
        .borrow()
        .get_total_size()
        .unwrap();
    let current_free = fs_size - current_used;
    let min_dir_size = required_space - current_free;

    let result: &RefCell<DirContent> = dir_sizes
        .values()
        .map(|dir| (dir, dir.borrow().get_total_size().unwrap()))
        .filter(|(_, size)| *size >= min_dir_size)
        .min_by(|(_, x_size), (_, y_size)| x_size.cmp(y_size))
        .map(|(dir, _)| dir)
        .unwrap();

    println!(
        "The file system is currently using {}, need to free at least {}, selected {}",
        current_used,
        min_dir_size,
        result.borrow().get_total_size().unwrap()
    );
}

fn compute_child_sizes(dir_sizes: &HashMap<String, RefCell<DirContent>>, for_dir: String) -> i32 {
    let dir_content = dir_sizes.get(&for_dir).unwrap();
    if let Some(s) = dir_content.borrow().get_total_size() {
        return s;
    }

    let child_total: i32 = dir_content
        .borrow()
        .children()
        .map(|child| compute_child_sizes(dir_sizes, child.clone()))
        .sum();

    let mut dir_content_mut = dir_content.borrow_mut();
    dir_content_mut.put_child_size(child_total);

    dir_content_mut.get_total_size().unwrap()
}

fn process_next_command(
    iter: &mut Peekable<Iter<String>>,
    dir_sizes: &mut HashMap<String, RefCell<DirContent>>,
    cwd: PathBuf,
) -> Option<PathBuf> {
    let next_command = iter.next().unwrap();

    match next_command[2..4].as_ref() {
        "cd" => match next_command[5..].as_ref() {
            ".." => {
                let mut new_cwd = cwd.clone();
                new_cwd.pop();
                Some(new_cwd)
            }
            name => Some(cwd.join(Path::new(&name.to_string()).to_owned())),
        },
        "ls" => {
            //println!("search dir {}", cwd);
            let mut dir_content = DirContent::new();
            loop {
                if let Some(next) = iter.peek() {
                    if next.starts_with("$") {
                        break;
                    }

                    let item = iter.next().unwrap();
                    match item[0..3].as_ref() {
                        "dir" => {
                            let child: String = item[4..].to_string();
                            let child_path = cwd.join(Path::new(&child));
                            dir_content.add_child(child_path.to_str().unwrap().to_string());
                        }
                        _ => {
                            let size = item.split(" ").next().unwrap();
                            dir_content.self_size_plus(size.parse::<i32>().unwrap());
                        }
                    }
                } else {
                    break;
                }
            }
            let old_value =
                dir_sizes.insert(cwd.to_str().unwrap().to_string(), RefCell::new(dir_content));
            if let Some(_) = old_value {
                panic!("value overwrite for {}", cwd.to_str().unwrap().to_string());
            }

            None
        }
        unknown_command => panic!("do not know how to handle command - {}", unknown_command),
    }
}

mod dir {
    use std::slice::Iter;

    pub struct DirContent {
        self_size: i32,
        children: Vec<String>,
        total_size: Option<i32>,
    }

    impl DirContent {
        pub fn new() -> Self {
            DirContent {
                self_size: 0,
                children: Vec::new(),
                total_size: None,
            }
        }

        pub fn self_size_plus(&mut self, size: i32) {
            self.self_size += size
        }

        pub fn add_child(&mut self, name: String) {
            self.children.push(name);
        }

        pub fn children(&self) -> Iter<'_, String> {
            self.children.iter()
        }

        pub fn is_leaf(&self) -> bool {
            self.children.is_empty()
        }

        pub fn put_child_size(&mut self, size: i32) {
            self.total_size = Some(self.self_size + size);
        }

        pub fn get_total_size(&self) -> Option<i32> {
            if self.is_leaf() {
                Some(self.self_size)
            } else {
                self.total_size
            }
        }
    }
}
