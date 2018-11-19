use std::io::prelude::*;
use std::io::{BufReader};
use std::fs;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

fn main() {
    let files_info = read_files_info();
    print_doubles(files_info);
}


struct FileInfo {
    name: String,
    size: u64
}

fn read_files_info() -> Vec<FileInfo> {
    let config_file = fs::File::open("./config.txt").expect("Failed to open config file");
    let reader = BufReader::new(config_file);
    let config_lines: Vec<String> = reader.lines().map(|l| { l.unwrap() }).collect();
    let work_folder = config_lines[0].clone();
    let entries = fs::read_dir(work_folder).expect("Failed to read work_dir files");

    let mut result = Vec::new();
    for entry in entries {
        let path = entry.unwrap().path();
        let name = String::from(path.file_name().unwrap().to_string_lossy());
        let size = fs::metadata(path).unwrap().len();
        result.push(FileInfo { name, size });
    }

    result
}

fn print_doubles(files_info: Vec<FileInfo>) {
    let mut doubles = Vec::new();
    let mut map = HashMap::new();
    for file in files_info {
        match map.entry(file.size) {
            Entry::Vacant(e) => {
               e.insert(vec![file.name]); 
            },
            Entry::Occupied(mut e) => {
                let mut names = e.get_mut();
                if names.len() == 1 {
                    doubles.push(file.size);
                } 

                names.push(file.name);
            }
        }
    }

    for d in doubles {
        let names = match map.get(&d) {
            Some(n) => n,
            _ => panic!()
        };
        println!("Item {} ------------------------", d);
        for n in names {
            println!("{}", n);
        }
        println!("------------------------------");
    }
}