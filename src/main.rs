use std::fs::ReadDir;
use std::fs::File;
use std::io::Read;
use term_size;
use colored::*;
use clap::Parser;
use json::JsonValue;

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

fn apply_style(style: &JsonValue, text: &str) -> ColoredString {
    let mut output = text.color(conv_color(style["color"].as_str().unwrap().to_string()));

    if style["bold"].as_bool().unwrap_or(false) {
        output = output.bold();
    }

    if style["underline"].as_bool().unwrap_or(false) {
        output = output.underline();
    }

    if style["reversed"].as_bool().unwrap_or(false) {
        output = output.reversed();
    }

    if style["italic"].as_bool().unwrap_or(false) {
        output = output.italic();
    }

    if style["blink"].as_bool().unwrap_or(false) {
        output = output.blink();
    }

    if style["hidden"].as_bool().unwrap_or(false) {
        output = output.hidden();
    }

    if style["strikethrough"].as_bool().unwrap_or(false) {
        output = output.strikethrough();
    }

    output
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

fn print_vec(vec: Vec<String>, file_type: String) {
    let (width, _) = term_size::dimensions().unwrap();
    let width = width -1;
    let mut current_line_length = 0;
    let json: json::JsonValue = parse_config()[file_type].clone();
    for i in &vec {
        let item_length = i.len() + 2;
        if current_line_length + item_length > width {
            println!();
            current_line_length = 0;
        }
        print!("{}  ", apply_style(&json, i));
        current_line_length += item_length;
    }
}

fn conv_color(color: String) -> Color {
    match color.as_str() {
        "red" => Color::Red,
        "blue" => Color::Blue,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "cyan" => Color::Cyan,
        "magenta" => Color::Magenta,
        "white" => Color::White,
        "black" => Color::Black,
        "bright_red" => Color::BrightRed,
        "bright_blue" => Color::BrightBlue,
        "bright_green" => Color::BrightGreen,
        "bright_yellow" => Color::BrightYellow,
        "bright_cyan" => Color::BrightCyan,
        "bright_magenta" => Color::BrightMagenta,
        "bright_white" => Color::BrightWhite,
        "bright_black" => Color::BrightBlack,
        _ => Color::TrueColor {r: color.split(",").nth(0).unwrap().parse::<u8>().unwrap(), g: color.split(",").nth(1).unwrap().parse::<u8>().unwrap(), b: color.split(",").nth(2).unwrap().parse::<u8>().unwrap()},
    }
}

fn parse_config() -> json::JsonValue{
    let exe_path = std::env::current_exe().unwrap();
    let exe_dir = exe_path.parent().unwrap();
    let config_path = exe_dir.join("config.json");
    let mut file = File::open(config_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let json: json::JsonValue = json::parse(&contents).unwrap();
    return json;
}

fn main() {
    
    let args = Args::parse();
    let items: ReadDir = std::fs::read_dir(args.path).unwrap();
    let (dirs, files) = sort_dirs(items, args.unordered, args.all);
    
    print_vec(dirs, "dir".to_string());
    println!();
    print_vec(files, "file".to_string());
    // println!(":a".color(Style::));
}
