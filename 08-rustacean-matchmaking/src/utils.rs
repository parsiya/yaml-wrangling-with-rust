use std::fs;
use walkdir::{DirEntry, WalkDir};

pub fn read_file_to_string(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    contents
}

// returns true if a DirEntry (file or directory) is hidden.
fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

// rule file extensions.
const RULE_EXTENSION: [&str; 2] = ["yml", "yaml"];
// file ending in `.test.yaml`, `.test.yml` and `.test.fixed.yaml` are test yaml files and not rules.
const TEST_EXTENSION: [&str; 3] = [".test.yml", ".test.yaml", ".test.fixed.yaml"];

// return the relative path to all files with RULE_EXTENSIOn in path (recursive).
pub fn find_rules(path: String) -> Vec<String> {

    let mut results: Vec<String> = Vec::new();

    let walker = WalkDir::new(path).into_iter();

    // ignore errors and skip hidden files/directories
    for entry in walker.filter_entry(|e| !is_hidden(e)).filter_map(|e| e.ok()) {

        let file_path = entry.path();

        // get the extension
        if let Some(extension) = file_path.extension() {
            // convert the extension to &str from &OsStr
            if let Some(ext_str) = extension.to_str() {
                // check if the file is a rule
                if RULE_EXTENSION.contains(&ext_str) {
                    // check if the file is yaml test file
                    // check if the file path ends with TEST_EXTENSION
                    let file_path_string = file_path.to_string_lossy();

                    // is this faster than iterating through TEST_EXTENSION?
                    if  file_path_string.ends_with(TEST_EXTENSION[0]) ||
                        file_path_string.ends_with(TEST_EXTENSION[1]) ||
                        file_path_string.ends_with(TEST_EXTENSION[2]) {
                            continue;
                    }
                    // println!("{:?} is a rule.", file_path.as_os_str());
                    results.push(file_path.to_string_lossy().as_ref().to_string());
                }
            }
        }
    }
    results
}

use std::path::Path;

// check if a path exists and if it's a directory, otherwise panic.
pub fn check_registry_path(path: &str) -> bool {

    let path_path = Path::new(&path);

    // check if path exists.
    if !Path::exists(path_path) {
        // return with an error.
        panic!("{} does not exist.", path);
    }

    // check if path is a directory. Technically, we could have just done this
    // check but we wouldn't know if the path existed vs. is not a directory.
    if !Path::is_dir(path_path) {
        // return with an error.
        panic!("{} is not a directory.", path);
    }
    true
}
