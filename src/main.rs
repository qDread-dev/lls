use std::fs::ReadDir;
use term_size;
use colored::*;
use clap::Parser;

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// Directory to list
    #[arg(default_value = ".")]
    path: String,
    /// Show / hide files starting with "."
    #[arg(short, default_value = "false")]
    all: bool,
    /// Don't sort directories and files
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

    let items: ReadDir = std::fs::read_dir(args.path).unwrap();
    
    let (dirs, files) = sort_dirs(items, args.unordered, args.all);
    let (width, _) = term_size::dimensions().unwrap();
    let width = width -1;
    let mut current_line_length = 0;
    
    for i in &dirs {
        let item_length = i.len() + 2;
        if current_line_length + item_length > width {
            println!();
            current_line_length = 0;
        }
        print!("{}  ", i.blue().bold());
        current_line_length += item_length;
    }
    println!();

    current_line_length = 0;
    for i in &files {
        let item_length: usize = i.len() + 2;
        if current_line_length + item_length > width {
            println!();
            current_line_length = 0;
        }
        print!("{}  ", i);
        current_line_length += item_length;
    }
    println!();
}
