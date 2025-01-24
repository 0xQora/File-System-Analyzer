use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{self, BufReader, Read},
    path::{Path, PathBuf},
    time::Instant,
};

use crate::model::{AnalyzeOptions, DirectorySummary};
use crate::{
    error::AnalysisError,
    model::{DuplicateGroup, FileInfoDirectory, FolderInfo},
};
use sha2::{Digest, Sha256};
use walkdir::{DirEntry, WalkDir};

pub fn directory_analyzer(option: &AnalyzeOptions) -> Result<DirectorySummary, AnalysisError> {
    let start_time = Instant::now();
    let path = option
        .path()
        .canonicalize()
        .map_err(|e| AnalysisError::IoError(e))?;

    if !path.exists() {
        return Err(AnalysisError::PathNotFound(path));
    }
    let mut size_groups: HashMap<u64, Vec<PathBuf>> = HashMap::new();

    let mut summary = DirectorySummary::new(
        0,
        0,
        0,
        0,
        std::time::Duration::default(),
        Vec::new(),
        Vec::new(),
        if option.detect_duplicates() {
            Some(Vec::new())
        } else {
            None
        },
    );

    let mut top_files = BinaryHeap::with_capacity(option.top_n());
    let mut folder_sizes: HashMap<PathBuf, u64> = HashMap::new();

    for entry in WalkDir::new(&path)
        .follow_links(option.follow_symlinks())
        .max_depth(option.max_depth().unwrap_or(usize::MAX))
        .into_iter()
        .filter_entry(|e| !option.should_ignore(e.path()))
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
            let size = metadata.len();

            if size >= option.min_size().unwrap_or_default() {
                summary.add_to_total_size(size);
                summary.increment_file_count();

                if option.detect_duplicates() {
                    size_groups
                        .entry(size)
                        .or_default()
                        .push(entry.path().to_path_buf());
                }

                top_files.push(FileInfoDirectory::new(entry.path().to_path_buf(), size));
                if top_files.len() > option.top_n() {
                    top_files.pop();
                }

                let mut current = entry
                    .path()
                    .parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_default();
                while current.starts_with(&path) {
                    *folder_sizes.entry(current.clone()).or_default() += size;
                    current = match current.parent() {
                        Some(p) => p.to_path_buf(),
                        None => break,
                    };
                }
            }
        } else if metadata.is_dir() {
            summary.increment_folder_count();
        } else if metadata.is_symlink() {
            summary.increment_symlink_count();
        }
    }

    let mut top_folders = BinaryHeap::new();
    for (path, size) in folder_sizes {
        top_folders.push(FolderInfo::new(path, size));
        if top_folders.len() > option.top_n() {
            top_folders.pop();
        }
    }

    if option.detect_duplicates() {
        //Calculate hash for files of the same size
        for (size, files) in size_groups.into_iter().filter(|(_, files)| files.len() > 1) {
            let mut hash_groups: HashMap<String, Vec<PathBuf>> = HashMap::new();
            for path in files {
                if let Ok(hash) = calculate_file_hash(&path) {
                    hash_groups.entry(hash).or_default().push(path);
                }
            }
            for (hash, paths) in hash_groups {
                if paths.len() > 1 {
                    summary
                        .duplicates_mut()
                        .as_mut()
                        .unwrap()
                        .push(DuplicateGroup::new(paths, size, hash));
                }
            }
        }
    }

    summary.set_largest_files(top_files.into_sorted_vec());
    summary.set_largest_folders(top_folders.into_sorted_vec());
    summary.set_duration(start_time.elapsed());

    Ok(summary)
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

fn calculate_file_hash(path: &Path) -> io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192]; // 8KB buffer

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}
