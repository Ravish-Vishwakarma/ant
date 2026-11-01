use std::{env, fs, path::PathBuf};

pub fn initilize_ant(args: &Vec<String>) {
    println!("Init with {:?}", args);
    let path = env::current_dir();

    match path {
        Ok(path) => create_folder(&path),
        Err(e) => println!("{}", e),
    }
}

fn create_folder(current_path: &PathBuf) {
    println!("Initializing ant in: {}", current_path.display());
    let create_dir = fs::create_dir_all(".ant");
    match create_dir {
        Ok(_) => {}
        Err(e) => println!("{e}"),
    }
}
