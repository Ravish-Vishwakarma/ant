use crate::{
    services,
    types::{Blob, GitObject},
};
use std::{fs, path::Path};

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
    if services::check_path_exists(&path) {
        let path = Path::new(&path);
        let bytes: Vec<u8> = fs::read(path).unwrap();
        let blob = Blob { content: bytes };
    } else {
        println!("{}, File doesn't exists", path);
    }
}

fn add_dir(path: &String) {
    println!("{} is a folder", path);
}

fn store_object(obj: GitObject) -> String {
    let obj_hash = obj.hash();
    let obj_dir = Path::new(".ant/objects/");
    let obj_dir = obj_dir.join(format!("{}", &obj_hash[0..2]));
    let obj_file = obj_dir.join(format!("{}", &obj_hash[2..]));
    if !obj_file.exists() {
        let created_dir = fs::create_dir_all(obj_dir);
        match created_dir {
            Ok(_) => {}
            Err(e) => println!("{e}"),
        }
        let serialized_content = obj.serialize();
        match serialized_content {
            Ok(content) => {
                let write_file = fs::write(obj_file, content);
                match write_file {
                    Ok(_) => {}
                    Err(e) => println!("{}", e),
                }
            }
            Err(e) => println!("{}", e),
        }
    } else {
        println!("Already exists")
    }
    return obj_hash;
}
