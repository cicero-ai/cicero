
use std::fs;
use std::path::Path;

/// Prepare directory
pub fn prepare_dir(dirname: &str) {

    if fs::metadata(dirname).is_ok() {
        fs::remove_dir_all(dirname).unwrap();
    }
    fs::create_dir(dirname).unwrap();
}

/// Get files and sub-directories
pub fn read_dir(dirname: &str) -> (Vec<String>, Vec<String>) {
    let mut dir = Path::new(&dirname);
    let (mut files, mut dirs) = (Vec::new(), Vec::new());

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                let file_name = file_name.to_string_lossy();

            // Skip the special entries "." and ".."
                if file_name == "." || file_name == ".." || file_name == "word_counters" {
                    continue;
                }

                let path = entry.path();
                if path.is_file() {
                    files.push(file_name.into_owned());
                } else if path.is_dir() {
                    dirs.push(file_name.into_owned());
                }
            }
        }
    }

    (files, dirs)
}



