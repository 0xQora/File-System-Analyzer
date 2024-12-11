use std::error::Error;

use crate::analyzer::directory_analyzer;
use crate::model;

pub fn display_directory_analyzer(option: model::AnalyzeOptions) -> Result<(), Box<dyn Error>> {
    let summary = directory_analyzer(&option)?;
    println!("📊 File System Analysis Report");
    println!("📂 Path: {}", option.path().display());
    println!(
        "⏱️  Scan completed in {:.1} seconds\n",
        summary.duration().as_secs_f64()
    );
    println!("Directory Summary:");
    println!("├── Total size: {}", format_size(summary.total_size()));
    println!("├── Files: {}", format_number(summary.file_count()));
    println!("├── Folders: {}", format_number(summary.folder_count()));
    println!("└── Symlinks: {}", format_number(summary.symlink_count()));

    println!("\nLargest Directories:");
    for (idx, dir) in summary.largest_folders().iter().enumerate() {
        let path_str = dir.path().display().to_string();
        let truncated_path = truncate_path(&path_str, 60);
        println!("{}. {} {}", idx + 1, truncated_path, format_size(dir.size()));
    }

    println!("\nLargest Files:");
    for (idx, file) in summary.largest_files().iter().enumerate() {
        let path_str = file.path().display().to_string();
        let truncated_path = truncate_path(&path_str, 60);
        println!("{}. {} {}", idx + 1, truncated_path, format_size(file.size()));
    }

    if option.detect_duplicates() {
        println!("\n Duplicates:");
        if let Some(duplicates) = summary.duplicates(){
            for (group_index,group) in duplicates.iter().enumerate(){
                println!("\tDuplicate #{} , Size= {}, Hash= {} :",group_index,format_size(group.size()),group.hash());
                for (file_index,file) in group.files().iter().enumerate(){
                    println!("\t\t{}. {} ",file_index,truncate_path(&file.as_path().display().to_string(),60));
                }
            }
        }
    }
    Ok(())
}


fn format_size(bytes: u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, units[unit_index])
}

fn truncate_path(path: &str, max_length: usize) -> String {
    if path.len() <= max_length {
        format!("{:<width$}", path, width = max_length)
    } else {
        let half = (max_length - 3) / 2;
        format!(
            "{}...{}",
            &path[..half],
            &path[path.len() - (max_length - half - 3)..]
        )
    }
}

fn format_number(num: u64) -> String {
    let num_str = num.to_string();
    let len = num_str.len();
    let mut formatted: Vec<char> = Vec::with_capacity(len + len / 3);

    for (i, c) in num_str.chars().enumerate() {
        if i > 0 && (len - i) % 3 == 0 {
            formatted.push(' ');
        }
        formatted.push(c);
    }

    formatted.into_iter().collect()
}

