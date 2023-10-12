# Folder Stats

This is a pretty unassuming project. I had to calculate some folder sizes at work, there were many folders, I wanted to practice
Rust, I had time, I wrote this.

It uses [walk_dir](https://github.com/BurntSushi/walkdir) to traverse the folder you give to it and prints back the total number
of files and also the number of files for each of the immediate subfolders present in the parent folder.

## Usage

Pretty straightforward:

```bash
$ folder_stats "/path/to/folder"
```

Either absolute or relative paths will work.

## Installation

For now, download or clone this repo and run `cargo build`. You'll then need to add the binary to your path or execute it specifying
the full path.

