use openssl::sha::Sha256;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub fn list_duplicates(dir_path: &PathBuf, include_hidden: &bool) {
    println!("Reading files from directory: {:?}", dir_path);

    let file_map = find_duplicates(&dir_path.as_path(), !include_hidden);
    for (file_hash, files) in file_map {
        if files.len() > 1 {
            println!("File hash {}", file_hash);
            for file in files {
                println!("{}", file.display());
            }
        }
    }
}

fn find_duplicates(directory: &Path, is_hidden: bool) -> HashMap<String, Vec<PathBuf>> {
    println!("including hidden files: {}", is_hidden);
    fs::read_dir(directory)
        .ok()
        .into_iter()
        .flat_map(|entries| entries)
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().is_file()
                && entry.file_name().to_string_lossy().starts_with('.') == is_hidden
        })
        .filter_map(|entry| {
            let file_path = entry.path();
            calculate_sha256(&file_path).map(|file_content_hash| (file_content_hash, file_path))
        })
        .fold(
            HashMap::new(),
            |mut file_map, (file_content_hash, file_path)| {
                file_map
                    .entry(file_content_hash)
                    .or_insert_with(Vec::new)
                    .push(file_path);
                file_map
            },
        )
}

fn calculate_sha256(file_path: &Path) -> Option<String> {
    File::open(file_path).ok().map(|file| {
        let reader = BufReader::new(file);
        let mut hasher = Sha256::new();

        for line in reader.lines().filter_map(|line| line.ok()) {
            hasher.update(line.as_bytes());
        }

        let hash_bytes = hasher.finish().to_vec();
        let hash_hex = hex::encode(hash_bytes);

        hash_hex
    })
}
