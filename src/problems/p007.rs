use crate::traits::Problem;
use std::fs;
use std::collections::HashMap;
use regex::Regex;

const MAX_SIZE: usize = 100000;
const TOTAL_SPACE: usize = 70000000;
const UNUSED_SPACE: usize = 30000000;

#[derive(Debug, Clone)]
pub struct P007 {
    input: String,
    size: Option<usize>
}

impl P007 {
    pub fn new() -> Self {
        P007 {
            input: fs::read_to_string("input/p007.txt").unwrap(),
            size: None
        }
    }
}

impl Problem<usize> for P007 {
    fn solve(&self) -> usize {
        if self.size.is_some() {
            return self.size.unwrap();
        }

        let input = self.input.clone();
        let file_system = FileSystem::from_input(input);

        let mut size = 0;
        let mut cache = HashMap::new();

        for dir in file_system.dirs.keys() {
            let dir_size = file_system.get_dir_size(dir, &mut cache);
            if dir_size <= MAX_SIZE {
                size += dir_size;
            }
        }

        size
    }

    fn phase2(&self) -> Self {
        let input = self.input.clone();
        let file_system = FileSystem::from_input(input);

        let mut cache = HashMap::new();

        for dir in file_system.dirs.keys() {
            file_system.get_dir_size(dir, &mut cache);
        }

        let root_size = cache.get("/").unwrap().clone();
        let size_infimum = root_size - (TOTAL_SPACE - UNUSED_SPACE);
        let mut smallest_size = root_size;

        for (_, size) in cache.iter() {
            if size >= &size_infimum {
                if size < &smallest_size {
                    smallest_size = *size;
                }
            }
        }

        P007 {
            input: self.input.clone(),
            size: Some(smallest_size)
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileSystem {
    dirs: HashMap<String, Dir>
}

#[derive(Debug, Clone)]
pub struct Dir {
    name: String,
    dirs: Box<HashMap<String, Dir>>,
    files: Vec<File>,
    size: Option<usize>
}

#[derive(Debug, Clone)]
pub struct File {
    _name: String,
    size: usize
}

impl File {
    pub fn new(_name: String, size: usize) -> Self {
        File {
            _name,
            size
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Dir {
    fn new(name: String) -> Self {
        Dir {
            name,
            dirs: Box::new(HashMap::new()),
            files: Vec::new(),
            size: None
        }
    }

    fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    fn add_dir(&mut self, name: &str) {
        let name = if self.name == "/" {
            format!("/{}", name)
        } else {
            format!("{}/{}", self.name, name)
        };
        let dir = Dir::new(name.clone());
        self.dirs.insert(name, dir);
    }

    fn get_size(&self) -> usize {
        if self.size.is_none() {
            let mut size = 0;
            for file in self.files.iter() {
                size += file.size();
            }
            for dir in self.dirs.values() {
                size += dir.get_size();
            }
            return size;
        } else {
            return self.size.unwrap();
        }
    }
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            dirs: HashMap::new()
        }
    }

    fn add_dir(&mut self, dir: &Dir) {
        self.dirs.insert(dir.name.clone(), dir.clone());
    }

    fn get_dir(&mut self, name: &str) -> &mut Dir {
        match self.dirs.get_mut(name) {
            Some(dir) => dir,
            None => panic!("{} not found in filesystem", name)
        }
    }

    fn get_dir_ref(&self, name: &str) -> &Dir {
        match self.dirs.get(name) {
            Some(dir) => dir,
            None => panic!("{} not found in filesystem", name)
        }
    }

    pub fn get_dir_size(&self, name: &str, cache: &mut HashMap<String, usize>) -> usize {
        if cache.contains_key(name) {
            return cache[name];
        }

        let mut size = 0;
        let dir = self.get_dir_ref(name);
        size += dir.get_size();

        for dir_name in dir.dirs.keys() {
            size += self.get_dir_size(dir_name, cache);
        }

        cache.insert(name.to_string(), size);

        size
    }

    pub fn from_input(input: String) -> Self {
        let mut file_system = FileSystem::new();
        let root_dir = Dir::new("/".to_string());
        file_system.add_dir(&root_dir);
        let mut curr_dir = file_system.get_dir("/");

        for line in input.lines().skip(1) {
            let re = Regex::new(r"\$\s(\w+)\s?(\w+|\W+)?").unwrap();
            match re.captures(line) {
                Some(caps) => {
                    let cmd = caps.get(1).unwrap().as_str();
                    if cmd == "cd" {
                        let dir = caps.get(2).unwrap().as_str();
                        if dir == ".." {
                            let curr_dir_name = curr_dir.name.clone();
                            let parent_dir_name_temp = curr_dir_name.chars()
                                .rev()
                                .skip_while(|c| *c != '/')
                                .skip(1)
                                .collect::<String>();
                            let parent_dir_name = if parent_dir_name_temp.is_empty() {
                                "/".to_string()
                            } else {
                                parent_dir_name_temp.chars().rev().collect::<String>()
                            };
                            curr_dir = file_system.get_dir(&parent_dir_name);
                        } else {
                            let dir_name = if curr_dir.name == "/" {
                                format!("/{}", dir)
                            } else {
                                format!("{}/{}", curr_dir.name, dir)
                            };
                            let dir = Dir::new(dir_name.clone());
                            file_system.add_dir(&dir);
                            curr_dir = file_system.get_dir(dir.name.as_str());
                        }
                    }
                }
                None => {
                    let re_dir = Regex::new(r"dir\s(\w+)").unwrap();
                    let re_file = Regex::new(r"(\d+)\s(\w+)").unwrap();

                    if re_dir.is_match(line) {
                        let dir = re_dir.captures(line).unwrap().get(1).unwrap().as_str();
                        curr_dir.add_dir(dir);
                    } else if re_file.is_match(line) {
                        let file = re_file.captures(line).unwrap();
                        let size = file.get(1).unwrap().as_str().parse::<usize>().unwrap();
                        let _name = file.get(2).unwrap().as_str();
                        let file = File::new(_name.to_string(), size);
                        curr_dir.add_file(file);
                    }
                }
            }
        }
        file_system
    }
}
