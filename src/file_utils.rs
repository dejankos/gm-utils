use io::Result;
use std::{fs, io};
use std::fs::read_dir;
use std::path::PathBuf;

pub fn read_file_content(file: &PathBuf) -> Result<String> {
    fs::read_to_string(file.as_path())
}

pub fn find_in_dir(dir: &PathBuf, file_name: &str) -> Option<PathBuf> {
    find(dir, file_name)
        .ok()
        .unwrap_or(None)
}

fn find(dir: &PathBuf, file_name: &str) -> Result<Option<PathBuf>> {
    let search_path = join_to_string(dir, file_name);
    for entry in read_dir(dir)? {
        let entry_path = entry.unwrap().path();
        let current = to_string(&entry_path);

        if current == search_path { // FIXME doesn't feel right
            return Ok(Some(entry_path));
        }
    }

    Ok(None)
}

fn join_to_string(dir: &PathBuf, file_name: &str) -> String {
    to_string(&dir.as_path().join(file_name))
}

fn to_string(dir: &PathBuf) -> String {
    dir.to_owned().into_os_string().into_string().unwrap()
}