use crate::services;
use crate::types::RepoFileType;
use std::{env, fs, path::PathBuf};

pub fn initilize_ant(args: &Vec<String>) {
    if services::check_ant_exists() {
        println!("Ant is already initialized");
    } else {
        println!("Init with {:?}", args);
        let path = env::current_dir();

        match path {
            Ok(path) => create_files_and_folders(&path),
            Err(e) => println!("{}", e),
        }
    }
}

fn create_files_and_folders(current_path: &PathBuf) {
    println!("Initializing ant in: {}", current_path.display());

    let folders = [
        ".ant",
        ".ant/objects/",
        ".ant/refs/heads/",
        // ".ant/refs/tags/",
        // ".ant/hooks/",
        // ".ant/info/",
        // ".ant/logs/",
    ];
    let files = [
        RepoFileType {
            path: String::from(".ant/index"),
            content: String::from("{}"),
        },
        RepoFileType {
            path: String::from(".ant/HEAD"),
            content: String::from("ref: refs/heads/master"),
        },
        // ".ant/index",
        // ".ant/HEAD",
        // ".ant/COMMIT_EDITMSG",
        // ".ant/description",
        // ".ant/config",
        // ".ant/info/exclude",
        // ".ant/logs/HEAD",
    ];
    for folder in folders {
        let create_dir = fs::create_dir_all(current_path.join(folder));
        match create_dir {
            Ok(_) => {}
            Err(e) => println!("{e}"),
        }
    }
    for file in files {
        let create_file = fs::write(format!("{}", file.path), file.content);
        match create_file {
            Ok(_) => {}
            Err(e) => println!("{e}"),
        }
    }
}
