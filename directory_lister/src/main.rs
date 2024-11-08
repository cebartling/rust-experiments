use chrono::{DateTime, Local};
use std::error::Error;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    // Default to current directory if no argument is provided
    let path = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    list_directory(&path)?;
    Ok(())
}

fn list_directory(path: &str) -> Result<(), Box<dyn Error>> {
    let path = Path::new(path);
    let entries = fs::read_dir(path)?;

    // Print header
    println!("\nContents of directory: {}\n", path.display());
    println!("{:<40} {:>12} {:<20}", "Name", "Size", "Modified");
    println!("{}", "-".repeat(74));

    for entry in entries {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let modified: DateTime<Local> = metadata.modified()?.into();

        // Get file name and handle non-UTF8 characters
        let name = entry
            .file_name()
            .into_string()
            .unwrap_or_else(|f| f.to_string_lossy().into_owned());

        // Format size
        let size = if metadata.is_dir() {
            "<DIR>".to_string()
        } else {
            format_size(metadata.len())
        };

        // Format modified date
        let modified = modified.format("%Y-%m-%d %H:%M:%S").to_string();

        // Print entry information
        println!("{:<40} {:>12} {:<20}", name, size, modified);
    }

    Ok(())
}

fn format_size(size: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];

    if size == 0 {
        return "0 B".to_string();
    }

    let size = size as f64;
    let base = 1024_f64;
    let exp = (size.ln() / base.ln()).floor() as i32;
    let exp = exp.min(UNITS.len() as i32 - 1);

    format!("{:.1} {}", size / base.powi(exp), UNITS[exp as usize])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_format_size_zero() {
        assert_eq!(format_size(0), "0 B");
    }

    #[test]
    fn test_format_size_bytes() {
        assert_eq!(format_size(512), "512.0 B");
    }

    #[test]
    fn test_format_size_kilobytes() {
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1536), "1.5 KB");
    }

    #[test]
    fn test_format_size_megabytes() {
        assert_eq!(format_size(1024 * 1024), "1.0 MB");
        assert_eq!(format_size(1024 * 1024 * 2), "2.0 MB");
    }

    #[test]
    fn test_format_size_gigabytes() {
        assert_eq!(format_size(1024 * 1024 * 1024), "1.0 GB");
    }

    #[test]
    fn test_list_directory_with_temp_files() -> Result<(), Box<dyn Error>> {
        // Create a temporary directory
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path();

        // Create some test files
        let file1_path = temp_path.join("test1.txt");
        let mut file1 = File::create(file1_path)?;
        file1.write_all(b"Hello, World!")?;

        let file2_path = temp_path.join("test2.txt");
        let mut file2 = File::create(file2_path)?;
        file2.write_all(b"Another test file")?;

        // Create a subdirectory
        fs::create_dir(temp_path.join("subdir"))?;

        // Test listing the directory
        let result = list_directory(temp_path.to_str().unwrap());
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn test_list_directory_nonexistent() {
        let result = list_directory("/path/that/does/not/exist");
        assert!(result.is_err());
    }

    #[test]
    fn test_list_directory_not_a_directory() -> Result<(), Box<dyn Error>> {
        // Create a temporary file
        let temp_dir = TempDir::new()?;
        let file_path = temp_dir.path().join("test.txt");
        let mut file = File::create(&file_path)?;
        file.write_all(b"Test content")?;

        // Try to list a file as if it were a directory
        let result = list_directory(file_path.to_str().unwrap());
        assert!(result.is_err());

        Ok(())
    }
}
