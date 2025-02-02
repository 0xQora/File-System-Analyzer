use std::{fs::File, io::{self, BufRead, BufReader}, path::Path, time::SystemTime};

use chrono::{DateTime, Local, NaiveDate, NaiveTime};

pub fn content_exists_in_file(file_path: &Path, search_string: &str) -> io::Result<Option<(usize, String)>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for (line_number, line_result) in reader.lines().enumerate() {
        let line = line_result?; // Handle potential IO errors
        if let Some(index) = line.find(search_string) {
            let max_length = 20;
            if line.len() <= max_length{
                
                return Ok(Some((line_number + 1, line))); 
            }
            let truncated_line = truncate_around_substring(&line, search_string, index, 50);
            return Ok(Some((line_number + 1, truncated_line))); 
        }
        if line.contains(search_string) {

            return Ok(Some((line_number + 1, line))); 
        }
    }

    Ok(None) // Pattern not found
}

pub fn format_size(bytes: &u64) -> String {
    let units = ["B", "KB", "MB", "GB", "TB"];
    let mut size = *bytes as f64; // Dereference the input
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < units.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.1} {}", size, units[unit_index])
}
pub fn truncate_path(path: &str, max_length: usize) -> String {
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

pub fn format_number(num: &u64) -> String {
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

pub fn format_datetime(time: &SystemTime) -> String {
    let datetime: DateTime<Local> = (*time).into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}


fn truncate_around_substring(s: &str, sub: &str,index: usize, max_length: usize) -> String {

    if s.len() <= max_length {
        return s.to_string();
    }
   
   let sub_len = sub.len();

    let available_context = max_length.saturating_sub(sub_len);
    let mut left_context = available_context / 2;
    let mut right_context = available_context - left_context;

    if left_context > index {
        let extra = left_context - index;
        left_context = index;
        right_context += extra;
    }
    
    let right_available = s.len() - (index + sub_len);
    if right_context > right_available {
        let extra = right_context - right_available;
        right_context = right_available;
        left_context = left_context.saturating_add(extra);
        if left_context > index {
            left_context = index;
        }
    }

    let start = index.saturating_sub(left_context);
    let end = (index + sub_len).saturating_add(right_context);

    let mut result = String::new();
    if start > 0 {
        result.push_str("...");
    }
    result.push_str(&s[start..end]);
    if end < s.len() {
        result.push_str("...");
    }

    result
}

pub fn parse_date(input: Option<String>, field: &str) -> Result<Option<SystemTime>, String> {
    input.map(|s| {
        let date = NaiveDate::parse_from_str(&s, "%Y-%m-%d")
            .map_err(|e| format!("Invalid {} date: {}. Expected format: YYYY-MM-DD", field, e))?;

        Ok(date.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
            .and_utc()
            .into())
    }).transpose()
}
