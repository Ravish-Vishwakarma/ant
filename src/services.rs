use std::path::Path;
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
