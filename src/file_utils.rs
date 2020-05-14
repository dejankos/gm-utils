use io::Result;
use std::ffi::OsString;
use std::fs::read_dir;
use std::path::PathBuf;
use std::{fs, io};

pub fn read_file_content(file: &PathBuf) -> Result<String> {
    fs::read_to_string(file.as_path())
}

pub fn find_in_dir(dir: &PathBuf, file_name: &str) -> Option<PathBuf> {
    find(dir, file_name).ok().unwrap_or(None)
}

fn find(dir: &PathBuf, file_name: &str) -> Result<Option<PathBuf>> {
    let search_path = join(dir, file_name);
    for entry in read_dir(dir)? {
        let entry_path = entry?.path();
        if entry_path.as_os_str() == search_path {
            return Ok(Some(entry_path));
        }
    }

    Ok(None)
}

fn join(dir: &PathBuf, file_name: &str) -> OsString {
    dir.as_path().join(file_name).into_os_string()
}
