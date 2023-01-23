use std::path::{Path, PathBuf};

pub fn get_external_drives() -> Vec<PathBuf> {
    let mut letters = vec![];

    for character in 'A'..='Z' {
        let path = PathBuf::from(format!(r"{character}:\"));
        if path.exists() && !path.starts_with("C:") {
            letters.push(path);
        }
    }

    letters
}

pub fn check_dir_entry(path: &Path) -> i8 {
    if path.is_dir() && !path.is_symlink() {
        0
    } else if path.is_file() && !path.is_symlink() {
        1
    } else {
        -1
    }
}
