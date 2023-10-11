use std::env;
use std::path::PathBuf;

use walkdir::{WalkDir, DirEntry};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct FileStats {
    file_count: i32,
    name: String
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let project_path = &args[1];
    let mut total_file_count: i32 = 0;
    let mut sub_directories: Vec<PathBuf> = Vec::new();
    let mut files_stats: Vec<FileStats> = Vec::new();

    for entry in WalkDir::new(project_path) {
        let dir_entry: DirEntry = entry.unwrap();
        let metadata = dir_entry.metadata().unwrap();
        if metadata.is_file() {
            total_file_count += 1;
        }

        if metadata.is_dir() && dir_entry.depth() == 1 {
            sub_directories.push(dir_entry.into_path());
        }
    }

    println!("Number of files: {}", total_file_count);

    for i in 0..sub_directories.len() {
        let mut file_count: i32 = 0;

        for entry in WalkDir::new(sub_directories[i].to_str().unwrap()) {
            let dir_entry: DirEntry = entry.unwrap();
            let metadata = dir_entry.metadata().unwrap();
            if metadata.is_file() {
                file_count += 1;
            }
        }

        let name = String::from(sub_directories[i].file_name().unwrap().to_str().unwrap());
        let file_stats = FileStats { file_count, name };

        files_stats.push(file_stats);
    }

    files_stats.sort();
    for i in (0..files_stats.len()).rev() {
        println!("Number of files in {}: {}", files_stats[i].name, files_stats[i].file_count);
    }

    return ();
}
