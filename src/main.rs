use chrono::*;
use colored::*;
use human_bytes::human_bytes;
use std::fs::{self, DirEntry, Metadata};
use std::path::Path;
use tabular::{Row, Table};

fn main() {
    let path = std::env::current_dir().unwrap();
    let items = list_path(&path);

    match items {
        Some(item) => display_paths(item, path.to_str().unwrap().to_string()),
        None => eprintln!("Failed to read directory"),
    };
}

fn list_path(path: &Path) -> Option<Vec<DirEntry>> {
    let items = fs::read_dir(path)
        .unwrap()
        .map(|f| f.unwrap())
        .collect::<Vec<DirEntry>>();

    Some(items)
}

fn display_paths(items: Vec<DirEntry>, root_path: String) {
    println!("Reading {root_path} items[{:?}]", items.len());

    let mut table = Table::new("{:<}  {:<}  {:<} {:<}");

    for item in items {
        let metadata = item.metadata().unwrap();

        let file_type_icon = get_file_type_icon_from_metadata(&metadata);
        let accessed: DateTime<Utc> = metadata.accessed().unwrap().into();

        table.add_row(
            Row::new()
                .with_cell(file_type_icon)
                .with_cell(item.file_name().to_str().unwrap().to_owned().white())
                .with_cell(format!("{}", accessed.format("%F")).bright_black())
                .with_cell(human_bytes(metadata.len() as f64).red()),
        );
    }
    println!("{}", &table)
}

fn get_file_type_icon_from_metadata(metadata: &Metadata) -> ColoredString {
    let mut icon: ColoredString = "?".bright_white();

    if metadata.is_dir() {
        icon = "\u{f413}".green()
    }

    if metadata.is_file() {
        icon = "\u{ea7b}".blue()
    }

    if metadata.is_symlink() {
        icon = "\u{ea9c}".yellow()
    }

    icon
}
