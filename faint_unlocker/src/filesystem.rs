use std::path;

pub fn get_external_drives() -> Vec<path::PathBuf> {
    let mut letters = vec![];

    for c in 'A'..='Z' {
        let path = path::PathBuf::from(format!(r"{c}:\"));
        if path.exists() && !path.starts_with("C:") {
            letters.push(path);
        }
    }

    letters
}

pub fn check_dir_entry(path: &path::Path) -> i8 {
    if path.is_dir() && !path.is_symlink() {
        0
    } else if path.is_file() && !path.is_symlink() {
        1
    } else {
        -1
    }
}
