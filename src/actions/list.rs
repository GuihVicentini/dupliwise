use openssl::sha::Sha256;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

pub fn list_duplicates(dir_path: &PathBuf, include_hidden: bool, depth: usize) {
    let file_map: HashMap<String, Vec<PathBuf>> =
        find_duplicates_recursivly(dir_path, include_hidden, depth);

    for (file_hash, files) in file_map {
        if files.len() > 1 {
            println!("File hash {}", file_hash);
            for file in files {
                println!("{}", file.display());
            }
        }
    }
}

pub fn find_duplicates_recursivly(
    root_dir: &Path,
    include_hidden: bool,
    depth: usize,
) -> HashMap<String, Vec<PathBuf>> {
    println!(
        "dir: {:?}, include hidden files: {}, depth: {}",
        root_dir, include_hidden, depth
    );

    let mut duplicated_map: HashMap<String, Vec<PathBuf>> = HashMap::new();

    let walker = WalkDir::new(root_dir)
        .max_depth(depth)
        .into_iter()
        .filter_entry(|e| filter_hidden_entry(e, include_hidden));

    for entry in walker {
        let ok_entry = match entry {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };

        if ok_entry.path().is_file() {
            let file_hash = calculate_sha256(&ok_entry.path());

            duplicated_map
                .entry(file_hash)
                .or_insert(Vec::new())
                .push(ok_entry.into_path());
        }
    }

    return duplicated_map;
}

fn filter_hidden_entry(entry: &DirEntry, include_hidden: bool) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| {
            if include_hidden {
                s.starts_with(".") || !s.starts_with(".")
            } else {
                !s.starts_with(".")
            }
        })
        .unwrap_or(false)
}

fn calculate_sha256(file_path: &Path) -> String {
    let file =
        File::open(file_path).expect(&format!("Failed to open file: {}", file_path.display()));
    let reader = BufReader::new(file);
    let mut hasher = Sha256::new();

    for line in reader.lines().filter_map(|line| line.ok()) {
        hasher.update(line.as_bytes());
    }

    let hash_bytes = hasher.finish().to_vec();
    let hash_hex = hex::encode(hash_bytes);

    return hash_hex;
}
