extern crate argparse;
extern crate colored;
extern crate fstream;
extern crate walkdir;

use argparse::{ArgumentParser, Store};
use colored::*;
use std::path::Path;
use walkdir::WalkDir;

fn check_dir(path: &str, query: &str) {
    let mut total_files_scanned = 0;
    for (fl_no, file) in WalkDir::new(path)
        .into_iter()
        .filter_map(|file| file.ok())
        .enumerate()
    {
        if file.metadata().unwrap().is_file() {
            match fstream::contains(file.path(), query) {
                Some(b) => {
                    if b {
                        check_file(file.path(), query);
                    }
                }
                None => println!("Error in walking Dir"),
            }
        }
        total_files_scanned = fl_no;
    }

    println!(
        "Total Scanned files {}",
        total_files_scanned.to_string().bold()
    );
}

fn check_file(file_path: &Path, query: &str) {
    println!(
        "In file {}\n",
        file_path.display().to_string().magenta().italic()
    );
    match fstream::read_lines(file_path) {
        Some(lines) => {
            for (pos, line) in &mut lines.iter().enumerate() {
                if line.contains(query) {
                    let line: String = line.trim().chars().take(2000).collect();
                    print!("{}", "Line ".green().bold());
                    print!("{0: <6} ", pos.to_string().cyan());
                    println!("=> {}", line.blue());
                }
            }
        }
        None => println!("Error in reading File"),
    }
    println!("");
}

fn main() {
    let mut path = ".".to_string();
    let mut query = "query".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Recursive string locater in files");
        ap.refer(&mut path)
            .add_option(&["-p", "--path"], Store, "Path to folder");
        ap.refer(&mut query)
            .add_option(&["-q", "--query"], Store, "Query string to find")
            .required();
        ap.parse_args_or_exit();
    }
    println!(
        "Searching '{}' in {}\n",
        query.green().bold(),
        path.italic()
    );
    check_dir(&path, &query);
}
