use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn check_ant_exists() -> bool {
    let ant_path = Path::new(".ant");
    if ant_path.exists() {
        return true;
    } else {
        return false;
    }
}

pub fn check_path_exists(path: &String) -> bool {
    let check_path = Path::new(path.as_str());
    if check_path.exists() {
        return true;
    } else {
        return false;
    }
}

pub fn get_all_files(dir: &Path) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| {
            if let Some(name) = e.file_name().to_str() {
                // name != ".ant" && name != "target"
                name != ".ant"
            } else {
                true
            }
        })
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .map(|e| e.into_path())
        .collect()
}
