use std::{env, fs, path::PathBuf};

pub fn initilize_ant(args: &Vec<String>) {
    println!("Init with {:?}", args);
    let path = env::current_dir();

    match path {
        Ok(path) => create_folders(&path),
        Err(e) => println!("{}", e),
    }
}

fn create_folders(current_path: &PathBuf) {
    println!("Initializing ant in: {}", current_path.display());

    let folders = [
        ".ant",
        ".ant/hooks/",
        ".ant/info/",
        ".ant/logs/",
        ".ant/objects/pack/",
        ".ant/refs/heads/",
        ".ant/refs/tags/",
    ];
    let files = [
        ".ant/index",
        ".ant/COMMIT_EDITMSG",
        ".ant/description",
        ".ant/HEAD",
        ".ant/config",
        ".ant/info/exclude",
        ".ant/logs/HEAD",
    ];
    for folder in folders {
        let create_dir = fs::create_dir_all(current_path.join(folder));
        match create_dir {
            Ok(_) => {}
            Err(e) => println!("{e}"),
        }
    }
    for file in files {
        let create_file = fs::write(format!("{}", file), "");
        match create_file {
            Ok(_) => {}
            Err(e) => println!("{e}"),
        }
    }
}
