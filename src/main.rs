use std::fs::ReadDir;

use colored::*;
use clap::Parser;

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, default_value = ".")]
    path: String,
    #[arg(short, default_value = "false")]
    all: bool,
    #[arg(short, default_value = "false")]
    unordered: bool,
}

fn sort_dirs(items: ReadDir, sort: bool, all: bool) -> (Vec<String>, Vec<String>) {
    let mut dirs: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();
    for i in items {
        // check if item is a directory or file
        let item = i.unwrap();
        let path = item.path();
        // check if file is hidden
        if (path.file_name().unwrap().to_str().unwrap().starts_with(".")) && !all {
            continue;
        }
        if path.is_dir() {
            dirs.push(item.file_name().into_string().unwrap());
        } else {
            files.push(item.file_name().into_string().unwrap());
        }
        if sort {
            return (dirs, files);
        }
        dirs.sort();
        files.sort();
    }
    return (dirs, files);
}

fn main() {

    let args = Args::parse();
    // set path to the 2nd arg, or default to "." if not provided
    let path = args.path;
    // get items in directory
    let items = std::fs::read_dir(path).unwrap();
    
    let (dirs, files) = sort_dirs(items, args.unordered, args.all);
    for i in &dirs {
        print!("{} ", i.green().bold());
    }
    for i in &files {
        print!("{} ", i.red().bold());
    }
}
