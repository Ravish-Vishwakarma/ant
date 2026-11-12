use crate::{services, types::Blob};
use serde_json;
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

fn add_file(file_path: &String) {
    println!("{} is a file", file_path);
    if services::check_path_exists(&file_path) {
        let path = Path::new(&file_path);
        let bytes: Vec<u8> = fs::read(path).unwrap();
        let blob = Blob::new(bytes);
        let blob_hash = store_object(blob);
        let mut index = load_index().unwrap();
        index[file_path] = serde_json::json!(blob_hash);
        let saved_index = save_index(&index);
        match saved_index {
            Ok(_) => println!("Added {:?}", path),
            Err(e) => println!("Error Saving Index File: {}", e),
        }
    } else {
        println!("{}, File doesn't exists", file_path);
    }
}

fn add_dir(dir_path: &String) {
    println!("{} is a folder", dir_path);
    if services::check_path_exists(&dir_path) {
        let all_files = services::get_all_files(Path::new(&dir_path));
        for file_path in &all_files {
            add_file(&file_path.display().to_string());
        }
        println!("Added {} files from path {}", all_files.len(), dir_path);
    } else {
        println!("{}, Directory doesn't exists", dir_path);
    }
}

fn save_index(index_content: &serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    let index_path = ".ant/index";
    let content = serde_json::to_string_pretty(&index_content)?;
    fs::write(index_path, content)?;
    Ok(())
}

fn load_index() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let index_path = String::from(".ant/index");

    if !services::check_path_exists(&index_path) {
        fs::write(&index_path, "{}")?;
    }

    let content = fs::read_to_string(&index_path)?;

    let json: serde_json::Value = serde_json::from_str(&content)?;

    Ok(json)
}

fn store_object(obj: Blob) -> String {
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
