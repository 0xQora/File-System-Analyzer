use std::error::Error;

use crate::analyzer::directory_analyzer;
use crate::model;
use crate::search::file_finder;
use crate::utils;

pub fn display_directory_analyzer(option: model::AnalyzeOptions) -> Result<(), Box<dyn Error>> {
    let summary = directory_analyzer(&option)?;
    println!("📊 File System Analysis Report");
    println!("📂 Path: {}", option.path().display());
    println!(
        "⏱️  Scan completed in {:.1} seconds\n",
        summary.duration().as_secs_f64()
    );
    println!("Directory Summary:");
    println!(
        "├── Total size: {}",
        utils::format_size(summary.total_size())
    );
    println!("├── Files: {}", utils::format_number(summary.file_count()));
    println!(
        "├── Folders: {}",
        utils::format_number(summary.folder_count())
    );
    println!(
        "└── Symlinks: {}",
        utils::format_number(summary.symlink_count())
    );

    println!("\nLargest Directories:");
    for (idx, dir) in summary.largest_folders().iter().enumerate() {
        let path_str = dir.path().display().to_string();
        let truncated_path = utils::truncate_path(&path_str, 60);
        println!(
            "{}. {} {}",
            idx + 1,
            truncated_path,
            utils::format_size(dir.size())
        );
    }

    println!("\nLargest Files:");
    for (idx, file) in summary.largest_files().iter().enumerate() {
        let path_str = file.path().display().to_string();
        let truncated_path = utils::truncate_path(&path_str, 60);
        println!(
            "{}. {} {}",
            idx + 1,
            truncated_path,
            utils::format_size(file.size())
        );
    }

    if option.detect_duplicates() {
        println!("\n Duplicates:");
        if let Some(duplicates) = summary.duplicates() {
            for (group_index, group) in duplicates.iter().enumerate() {
                println!(
                    "\tDuplicate #{} , Size= {}, Hash= {} :",
                    group_index,
                    utils::format_size(group.size()),
                    group.hash()
                );
                for (file_index, file) in group.files().iter().enumerate() {
                    println!(
                        "\t\t{}. {} ",
                        file_index,
                        utils::truncate_path(&file.as_path().display().to_string(), 60)
                    );
                }
            }
        }
    }
    Ok(())
}

pub fn display_search_result(options: model::SearchOptions) -> Result<(), Box<dyn Error>> {
    let result = file_finder(&options)?;
    if options.content_pattern().is_some() {
        display_content_search(&options, &result)
    } else {
        display_simple_search(&options, &result)
    }
}

fn display_simple_search(
    options: &model::SearchOptions,
    result: &model::SearchResult,
) -> Result<(), Box<dyn Error>> {
    println!(
        "🔍 Search Results ({} matches):",
        result.files_result().len()
    );

    // Display search criteria
    if !options.name_pattern().is_empty() {
        for pattern in options.name_pattern() {
            println!("└── Pattern: \"{}\"", pattern);
        }
    }
    if let Some(date) = &options.modified_after() {
        println!("└── Modified after: {:#?}", date);
    }
    if let Some(date) = &options.modified_before() {
        println!("└── Modified before: {:#?}", date);
    }
    if let Some(min) = options.min_size() {
        println!("└── Min size: {}", utils::format_size(min));
    }
    if let Some(max) = options.max_size() {
        println!("└── Max size: {}", utils::format_size(max));
    }
    println!();

    // Display files
    for file in result.files_result() {
        println!("{}", file.path().display());
        println!("├── Size: {}", utils::format_size(file.size()));
        println!(
            "└── Modified: {}",
            utils::format_datetime(file.modified_date())
        );
        println!();
    }

    // Display summary
    println!("📊 Summary:");
    println!("├── Files found: {}", result.files_result().len());
    println!(
        "├── Total size: {}",
        utils::format_size(result.total_size())
    );
    println!(
        "└── Search time: {:.1}s",
        result.search_time().as_secs_f32()
    );

    Ok(())
}

fn display_content_search(
    _options: &model::SearchOptions,
    result: &model::SearchResult,
) -> Result<(), Box<dyn Error>> {
    println!(
        "🔍 Content Search Results ({} matches):\n",
        result.files_result().len()
    );

    // Display matches
    for file in result.files_result() {
        if let Some((line_num, content)) = file.content() {
            let path_str = file.path().display().to_string();
            println!("{}:{} - {}", utils::truncate_path(&path_str, 30), line_num, content);
        }
    }

    // Display summary
    println!("\n📊 Summary:");
    println!("├── Files searched: {}", result.file_searched());
    println!("├── Matches found: {}", result.files_result().len());
    println!(
        "└── Search time: {:.1}s",
        result.search_time().as_secs_f32()
    );

    Ok(())
}
