use std::time::Instant;

use walkdir::{DirEntry, WalkDir};

use crate::error::AnalysisError;
use crate::model::{FileInfoSearch, SearchOptions, SearchResult};

pub fn file_finder(option: &SearchOptions) -> Result<SearchResult, AnalysisError> {
    let start_time: Instant = Instant::now();
    let path = option
        .path()
        .canonicalize()
        .map_err(|e| AnalysisError::IoError(e))?;

    if !path.exists() {
        return Err(AnalysisError::PathNotFound(path));
    }

    let mut search_result: SearchResult =
        SearchResult::new(0, 0, std::time::Duration::default(), Vec::new());
    let mut file_result: Vec<FileInfoSearch> = Vec::new();

    for entry in WalkDir::new(&path)
        .follow_links(false)
        .into_iter()
        .filter_map(handle_dir_entry)
    {
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(e) => {
                eprintln!(
                    "Warning: Cannot read metadata for {}: {}",
                    entry.path().display(),
                    e
                );
                continue;
            }
        };

        if metadata.is_file() {
            search_result.increment_file_searched();

            if !option.match_name_pattern(entry.path()) {
                continue;
            }
            let (matched ,content) = option.match_content_pattern(entry.path());
            if  !matched{
                continue;
            }
            let modified_time = metadata.modified().unwrap();
            if !option.match_modified_date(&modified_time){
                continue;
            }
            let size = metadata.len();
            if !option.match_size(&size){
                continue;
            }
            
            file_result.push(FileInfoSearch::new(entry.path().to_path_buf(), size, content, modified_time));
            search_result.add_to_total_size(size);
        }

    }
    search_result.set_files_result(file_result);
    search_result.set_duration(start_time.elapsed());

    Ok(search_result)
}

fn handle_dir_entry(entry: Result<DirEntry, walkdir::Error>) -> Option<DirEntry> {
    match entry {
        Ok(entry) => Some(entry),
        Err(e) => {
            eprintln!("Warning: {}", e);
            None
        }
    }
}
