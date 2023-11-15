use std::{env, fmt::Arguments};
use colored::*;
use clap::Parser;

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about=None)]
struct args {
    #[arg(short)]
    path: String,
}

fn main() {
    println!("Hello, world!");
    let args = args::parse();
    dbg!(&args);
    // set path to the 2nd arg, or default to "." if not provided
    let path = ".";
    // get items in directory
    let items = std::fs::read_dir(path).unwrap();
    let mut dirs: Vec<String> = Vec::new();
    let mut files: Vec<String> = Vec::new();
    for i in items {
        // check if item is a directory or file
        let item = i.unwrap();
        let path = item.path();
        if path.is_dir() {
            dirs.push(item.file_name().into_string().unwrap());
        } else {
            files.push(item.file_name().into_string().unwrap());
        }
    }
    dbg!(&dirs);
    dbg!(&files);
    for i in &dirs {
        print!("{} ", i.green().bold());
    }
    for i in &files {
        print!("{} ", i.red().bold());
    }
}
