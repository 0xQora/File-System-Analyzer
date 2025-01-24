use chrono::{NaiveDate, NaiveDateTime};
use clap::builder::styling::Reset;
use clap::error::Result;
use glob::Pattern;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub struct FileInfoDirectory {
    path: PathBuf,
    size: u64,
}

#[derive(Debug, Eq, PartialEq)]
pub struct FolderInfo {
    path: PathBuf,
    size: u64,
}

#[derive(Debug)]
pub struct DuplicateGroup {
    files: Vec<PathBuf>,
    size: u64,
    hash: String,
}
#[derive(Debug)]
pub struct DirectorySummary {
    total_size: u64,
    file_count: u64,
    folder_count: u64,
    symlink_count: u64,
    duration: std::time::Duration,
    largest_files: Vec<FileInfoDirectory>,
    largest_folders: Vec<FolderInfo>,
    duplicates: Option<Vec<DuplicateGroup>>,
}

// FileInfo implementations
impl FileInfoDirectory {
    pub fn new(path: PathBuf, size: u64) -> Self {
        FileInfoDirectory { path, size }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}

// FolderInfo implementations
impl FolderInfo {
    pub fn new(path: PathBuf, size: u64) -> Self {
        FolderInfo { path, size }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn size(&self) -> u64 {
        self.size
    }
}

impl DuplicateGroup {
    pub fn new(files: Vec<PathBuf>, size: u64, hash: String) -> Self {
        DuplicateGroup { files, size, hash }
    }

    pub fn files(&self) -> &Vec<PathBuf> {
        &self.files
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }
}

impl DirectorySummary {
    pub fn new(
        total_size: u64,
        file_count: u64,
        folder_count: u64,
        symlink_count: u64,
        duration: std::time::Duration,
        largest_files: Vec<FileInfoDirectory>,
        largest_folders: Vec<FolderInfo>,
        duplicates: Option<Vec<DuplicateGroup>>,
    ) -> Self {
        DirectorySummary {
            total_size,
            file_count,
            folder_count,
            symlink_count,
            duration,
            largest_files,
            largest_folders,
            duplicates,
        }
    }

    pub fn total_size(&self) -> u64 {
        self.total_size
    }

    pub fn file_count(&self) -> u64 {
        self.file_count
    }

    pub fn folder_count(&self) -> u64 {
        self.folder_count
    }

    pub fn symlink_count(&self) -> u64 {
        self.symlink_count
    }

    pub fn duration(&self) -> std::time::Duration {
        self.duration
    }

    pub fn largest_files(&self) -> &Vec<FileInfoDirectory> {
        &self.largest_files
    }

    pub fn largest_folders(&self) -> &Vec<FolderInfo> {
        &self.largest_folders
    }

    pub fn duplicates_mut(&mut self) -> &mut Option<Vec<DuplicateGroup>> {
        &mut self.duplicates
    }
    pub fn duplicates(&self) -> &Option<Vec<DuplicateGroup>> {
        &self.duplicates
    }
    pub fn add_to_total_size(&mut self, size: u64) {
        self.total_size += size;
    }

    pub fn increment_file_count(&mut self) {
        self.file_count += 1;
    }

    pub fn increment_folder_count(&mut self) {
        self.folder_count += 1;
    }

    pub fn increment_symlink_count(&mut self) {
        self.symlink_count += 1;
    }

    pub fn set_duration(&mut self, duration: std::time::Duration) {
        self.duration = duration;
    }

    pub fn set_largest_files(&mut self, files: Vec<FileInfoDirectory>) {
        self.largest_files = files;
    }

    pub fn set_largest_folders(&mut self, folders: Vec<FolderInfo>) {
        self.largest_folders = folders;
    }
}

pub struct AnalyzeOptions {
    path: PathBuf,
    max_depth: Option<usize>,
    top_n: usize,
    follow_symlinks: bool,
    min_size: Option<u64>,
    detect_duplicates: bool,
    ignore_patterns: Vec<Pattern>,
}

impl AnalyzeOptions {
    pub fn new(
        path: PathBuf,
        max_depth: Option<usize>,
        top_n: usize,
        follow_symlinks: bool,
        min_size_string: Option<String>,
        detect_duplicates: bool,
        ignore_patterns: Vec<String>,
    ) -> Result<AnalyzeOptions, String> {
        // Convert string patterns to compiled glob patterns
        let ignore_patterns = ignore_patterns
            .into_iter()
            .filter_map(|p| Pattern::new(&p).ok())
            .collect();

        // Convert readable size into size in bytes
        let min_size: Option<u64> = match min_size_string {
            Some(input) => {
                let input = input.trim();
                if let Ok(bytes) = u64::from_str(input) {
                    Some(bytes)
                } else {
                    let suffixes: [(&str, u64); 4] = [
                        ("KB", 1024),
                        ("MB", 1024 * 1024),
                        ("GB", 1024 * 1024 * 1024),
                        ("TB", 1024 * 1024 * 1024 * 1024),
                    ];

                    let mut calculated_size: Option<u64> = None;

                    for (suffix, multiplier) in suffixes.iter() {
                        if input.ends_with(suffix) {
                            let number_part = &input[..input.len() - suffix.len()].trim();
                            if let Ok(number) = number_part.parse::<f64>() {
                                calculated_size =
                                    Some((number * *multiplier as f64).round() as u64);
                                break;
                            } else {
                                return Err(format!("Invalid number part: {}", number_part));
                            }
                        }
                    }

                    // If no valid size is calculated, return an error
                    match calculated_size {
                        Some(size) => Some(size),
                        _none => {
                            return Err("Invalid size format".to_string());
                        }
                    }
                }
            }
            _none => None, // No size provided, keep it as None
        };

        // Return the final AnalyzeOptions object
        Ok(AnalyzeOptions {
            path,
            max_depth,
            top_n,
            follow_symlinks,
            min_size,
            detect_duplicates,
            ignore_patterns,
        })
    }
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn max_depth(&self) -> Option<usize> {
        self.max_depth
    }

    pub fn top_n(&self) -> usize {
        self.top_n
    }

    pub fn follow_symlinks(&self) -> bool {
        self.follow_symlinks
    }

    pub fn min_size(&self) -> Option<u64> {
        self.min_size
    }

    pub fn detect_duplicates(&self) -> bool {
        self.detect_duplicates
    }

    pub fn should_ignore(&self, path: &Path) -> bool {
        if let Some(path_str) = path.to_str() {
            self.ignore_patterns
                .iter()
                .any(|pattern| pattern.matches(path_str))
        } else {
            false
        }
    }
}

// Ordering for heap operations
impl Ord for FileInfoDirectory {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.size.cmp(&self.size) // Reverse order for max-heap
    }
}

impl PartialOrd for FileInfoDirectory {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FolderInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.size.cmp(&self.size) // Reverse order for max-heap
    }
}

impl PartialOrd for FolderInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub enum FileType {
    File,
    Dir,
    Sym,
}

impl FromStr for FileType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "file" => Ok(FileType::File),
            "dir" | "directory" => Ok(FileType::Dir),
            "sym" | "symlink" | "link" => Ok(FileType::Sym),
            _ => Err(format!("Invalid file type: '{}'. Valid values are: file, dir, sym", s)),
        }
    }
}


pub struct SearchOptions {
    path: PathBuf,
    name_pattern: Vec<Pattern>,
    content_pattern: Option<String>,
    modified_after: Option<NaiveDateTime>,
    modified_before: Option<NaiveDateTime>,
    min_size: Option<u64>,
    max_size: Option<u64>,
    file_type: Option<FileType>,
}

impl SearchOptions {
    pub fn new(
        path: PathBuf,
        name_pattern: Vec<String>,
        content_pattern: Option<String>,
        modified_after: Option<NaiveDate>,
        modified_before: Option<NaiveDate>,
        min_size: Option<u64>,
        max_size: Option<u64>,
        file_type: Option<String>,
    ) -> Result<Self, String> {
        // Convert name patterns with proper error handling
        let name_pattern = name_pattern
            .into_iter()
            .map(|p| Pattern::new(&p).map_err(|e| format!("Invalid pattern '{}': {}", p, e)))
            .collect::<Result<Vec<_>, _>>()?;

        // Convert file type string to enum
        let file_type = match file_type {
            Some(s) => Some(FileType::from_str(&s)?),
            None => None,
        };

        Ok(Self {
            path,
            name_pattern,
            content_pattern,
            modified_after,
            modified_before,
            min_size,
            max_size,
            file_type,
        })
    }

    // Getters
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn is_pattern(&self, path: &Path) -> bool {
        if let Some(path_str) = path.to_str() {
            self.name_pattern
                .iter()
                .any(|pattern| pattern.matches(path_str))
        } else {
            false
        }
    }

    pub fn content_pattern(&self) -> Option<&String> {
        self.content_pattern.as_ref()
    }

    pub fn modified_after(&self) -> Option<&NaiveDateTime> {
        self.modified_after.as_ref()
    }

    pub fn modified_before(&self) -> Option<&NaiveDateTime> {
        self.modified_before.as_ref()
    }

    pub fn min_size(&self) -> Option<u64> {
        self.min_size
    }

    pub fn max_size(&self) -> Option<u64> {
        self.max_size
    }

    pub fn file_type(&self) -> Option<&FileType> {
        self.file_type.as_ref()
    }
}

pub struct FileInfoSearch {
    path: PathBuf,
    size: u64,
    content: Option<(u32, String)>,
    modified_date: Option<NaiveDateTime>,
}
impl FileInfoSearch {
    pub fn new(
        path: PathBuf,
        size: u64,
        content: Option<(u32, String)>,
        modified_date: Option<NaiveDateTime>,
    ) -> Self {
        FileInfoSearch {
            path,
            size,
            content,
            modified_date,
        }
    }
}

pub struct SearchResult {
    total_size: u64,
    file_searched: u64,
    search_time: std::time::Duration,
    files_result: Vec<FileInfoSearch>,
}
impl SearchResult {
    pub fn new(
        total_size: u64,
        file_searched: u64,
        search_time: std::time::Duration,
        files_result: Vec<FileInfoSearch>,
    ) -> Self {
        SearchResult {
            total_size,
            file_searched,
            search_time,
            files_result,
        }
    }

    pub fn add_to_total_size(&mut self, size: u64) {
        self.total_size += size;
    }

    pub fn increment_file_searched(&mut self) {
        self.file_searched += 1;
    }
}
