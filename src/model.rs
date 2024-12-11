use std::path::{Path, PathBuf};
use glob::Pattern;

#[derive(Debug, Eq, PartialEq)]
pub struct FileInfo {
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
    largest_files: Vec<FileInfo>,
    largest_folders: Vec<FolderInfo>,
    duplicates: Option<Vec<DuplicateGroup>>,
}

// FileInfo implementations
impl FileInfo {
    pub fn new(path: PathBuf, size: u64) -> Self {
        FileInfo { path, size }
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
        largest_files: Vec<FileInfo>,
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

    pub fn largest_files(&self) -> &Vec<FileInfo> {
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
 
    pub fn set_largest_files(&mut self, files: Vec<FileInfo>) {
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
        min_size: Option<u64>,
        detect_duplicates: bool,
        ignore_patterns: Vec<String>,
    ) -> AnalyzeOptions {
        // Convert string patterns to compiled glob patterns
        let ignore_patterns = ignore_patterns
            .into_iter()
            .filter_map(|p| Pattern::new(&p).ok())
            .collect();

        AnalyzeOptions {
            path,
            max_depth,
            top_n,
            follow_symlinks,
            min_size,
            detect_duplicates,
            ignore_patterns,
        }
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
impl Ord for FileInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.size.cmp(&self.size) // Reverse order for max-heap
    }
}

impl PartialOrd for FileInfo {
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
