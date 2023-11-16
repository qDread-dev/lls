use std::fs::ReadDir;
use std::fs::File;
use std::io::Read;
use term_size;
use colored::*;
use clap::Parser;
use json::JsonValue;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use regex::Regex;
use rayon::prelude::*;
use std::fmt::Write;
use std::fs::metadata;

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
    /// Show files in sub directories
    #[arg(short, default_value = "false")]
    recursive: bool,
    /// Regex to match files
    #[arg(long, default_value = "")]
    regex: String,
}
#[derive(Clone)]
struct PathMeta {
    path: PathBuf,
    metadata: fs::Metadata,
}
#[inline]
fn apply_style(style: &JsonValue, text: String, readonly: bool) -> ColoredString {
    let mut output = text.color(conv_color(style["color"].as_str().unwrap().to_string()));
    if readonly {
        output = output.color(conv_color(style["readonly_color"].as_str().unwrap().to_string()));
        output = output.on_color(conv_color(style["readonly_background_color"].as_str().unwrap().to_string()));
    }
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

fn sort_dirs(items: ReadDir, args: Args) -> Result<(Vec<PathBuf>, Vec<PathBuf>), regex::Error> {
    let regex = Regex::new(&args.regex)?;

    let mut paths: Vec<_> = items
        .filter_map(Result::ok)
        .map(|entry| {
            let path = entry.path();
            let metadata = fs::metadata(&path).unwrap();
            PathMeta { path, metadata }
        })
        .collect();
    paths.par_sort_by(|a, b| {
        let a_file_type = determine_file_type(&a.path, &a.metadata);
        let b_file_type = determine_file_type(&b.path, &b.metadata);

        a_file_type.cmp(&b_file_type)
    });

    let (dirs, files): (Vec<_>, Vec<_>) = paths
        .par_iter()
        .filter(|path_meta| {
            // Include hidden files/directories if args.all is true
            args.all || path_meta.path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| !name.starts_with('.'))
                .unwrap_or(false)
        })
        .partition(|path_meta| {
            path_meta.path.is_dir() && path_meta.path.to_str().map_or(false, |s| regex.is_match(s))
        });
        let dirs: Vec<PathBuf> = dirs.into_iter().map(|path_meta| path_meta.path.clone()).collect();
        let files: Vec<PathBuf> = files.into_iter().map(|path_meta| path_meta.path.clone()).collect();
    Ok((dirs, files))
}

fn determine_file_type(path: &PathBuf, metadata: &fs::Metadata) -> String {
    if path.is_dir() {
        "dir"
    } else if metadata.permissions().readonly() {
        "readonly"
    } else {
        "file"
    }.to_string()
}


fn print_vec(mut vec: Vec<PathBuf>, file_type: String) {
    let (width, _) = term_size::dimensions().unwrap();
    let width = width - 1;
    let config = parse_config();
    let json = &config[file_type];
    vec.sort();
    let max_item_length = vec.iter()
        .map(|path| path.file_name().unwrap_or_else(|| path.as_os_str()).to_str().unwrap().len())
        .max()
        .unwrap_or(0); // +2 for the spaces after each item

    let num_columns = width / max_item_length;

    let mut output = String::new();
    for (i, path) in vec.iter().enumerate() {
        let item = match path.strip_prefix("./") {
            Ok(stripped) => stripped.to_str().unwrap(),
            Err(_) => path.file_name().unwrap_or_else(|| path.as_os_str()).to_str().unwrap(),
        };
    
        let metadata: fs::Metadata = metadata(path).unwrap();    
        let styled_item = apply_style(&json, item.to_string(), metadata.permissions().readonly());
    
        if i % num_columns == 0 && i != 0 {
            write!(output, "\n").unwrap();
        }
    
        let formatted_item = format!("{:width$}", styled_item, width = max_item_length);
        write!(output, "{}", formatted_item).unwrap();
    }
    writeln!(output).unwrap();
    print!("{}", output);
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

fn recursive_read(dir: &Path, regex: Regex) -> std::io::Result<Vec<String>> {
    let mut files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let regex_clone = regex.clone();
            files.extend(recursive_read(&path, regex_clone)?);
        } else {
            if !regex.is_match(path.to_str().unwrap()) {
                continue;
            }
            files.push(String::from(path.to_str().unwrap()));
        }
    }
    Ok(files)
}

fn main() {
    let args = Args::parse();
    if !args.recursive {
        let items: ReadDir = std::fs::read_dir(&args.path).unwrap();
        let (dirs, files) = match sort_dirs(items, args) {
            Ok((dirs, files)) => (dirs, files),
            Err(e) => panic!("Error: {}", e),
        };
        print_vec(dirs, "dir".to_string());
        println!();
        print_vec(files, "file".to_string());
    } else {
        match recursive_read(Path::new(&args.path), args.regex.parse::<Regex>().unwrap()) {
            Ok(files) => {
                for file in files {
                    println!("{}", file);
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}