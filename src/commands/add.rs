use crate::services;
use std::path::Path;

pub fn ant_add(args: &Vec<String>) {
    if services::check_ant_exists() {
        if args.len() < 3 {
            println!("Please enter a file or path to add.");
        } else {
            for arg in &args[2..] {
                if services::check_path_exists(&arg) {
                    let path = Path::new(arg);
                    if path.is_file() {
                        add_file(&arg);
                    } else if path.is_dir() {
                        add_dir(&arg);
                    } else {
                        println!("{} is neither a file nor a directory", arg);
                    }
                } else {
                    println!("Path doesn't exists.");
                }
            }
        }
    } else {
        println!("Repo isn't initlized.");
    }
}

fn add_file(path: &String) {
    println!("{} is a file", path);
}

fn add_dir(path: &String) {
    println!("{} is a folder", path);
}
