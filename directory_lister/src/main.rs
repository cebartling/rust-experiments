use chrono::{DateTime, Local};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct ListingOptions {
    recursive: bool,
    indent_level: usize,
}

fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

fn print_header(path: &Path) {
    println!("\nContents of directory: {}", path.display());
    println!("{:-<80}", "");
    println!("{:<40} {:>12} {:>25}", "Name", "Size", "Modified");
    println!("{:-<80}", "");
}

fn list_directory(path: &Path, options: &ListingOptions) -> Result<(), Box<dyn Error>> {
    if options.indent_level == 0 {
        print_header(path);
    }

    let entries = fs::read_dir(path)?;
    let mut entries: Vec<_> = entries.collect::<Result<_, _>>()?;

    entries.sort_by(|a, b| {
        let a_metadata = a.metadata().unwrap();
        let b_metadata = b.metadata().unwrap();
        let a_is_dir = a_metadata.is_dir();
        let b_is_dir = b_metadata.is_dir();

        match (a_is_dir, b_is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        }
    });

    for entry in entries {
        let metadata = entry.metadata()?;
        let modified: DateTime<Local> = metadata.modified()?.into();
        let name = entry.file_name().to_string_lossy().into_owned();

        let indent = "    ".repeat(options.indent_level);

        let size = if metadata.is_file() {
            format_size(metadata.len())
        } else {
            String::from("<DIR>")
        };

        let max_name_length = 40 - (options.indent_level * 4);
        let displayed_name = if name.len() > max_name_length && max_name_length > 3 {
            format!("{}...", &name[..max_name_length-3])
        } else {
            name.clone()
        };

        println!("{}{:<40} {:>12} {:>25}",
                 indent,
                 displayed_name,
                 size,
                 modified.format("%Y-%m-%d %H:%M:%S")
        );

        if options.recursive && metadata.is_dir() {
            let new_path = PathBuf::from(entry.path());
            let new_options = ListingOptions {
                recursive: true,
                indent_level: options.indent_level + 1,
            };

            if let Err(e) = list_directory(&new_path, &new_options) {
                eprintln!("{}Error accessing {}: {}", indent, new_path.display(), e);
            }
        }
    }

    Ok(())
}

fn main() {
    let mut args = std::env::args().skip(1);
    let mut path = None;
    let mut recursive = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-r" | "--recursive" => recursive = true,
            _ => path = Some(PathBuf::from(arg)),
        }
    }

    let path = path.unwrap_or_else(|| std::env::current_dir().unwrap());
    let options = ListingOptions {
        recursive,
        indent_level: 0,
    };

    match list_directory(&path, &options) {
        Ok(_) => (),
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    // Helper function to create a test directory structure
    fn create_test_directory() -> TempDir {
        let temp_dir = TempDir::new().unwrap();

        // Create some files
        let file1_path = temp_dir.path().join("test1.txt");
        let mut file1 = File::create(file1_path).unwrap();
        file1.write_all(b"Hello World").unwrap();

        let file2_path = temp_dir.path().join("test2.txt");
        let mut file2 = File::create(file2_path).unwrap();
        file2.write_all(b"This is a test").unwrap();

        // Create a subdirectory with files
        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();

        let subfile_path = subdir.join("subtest.txt");
        let mut subfile = File::create(subfile_path).unwrap();
        subfile.write_all(b"Subdirectory test").unwrap();

        temp_dir
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_size(1024 * 1024 * 1024), "1.00 GB");
    }

    #[test]
    fn test_directory_exists() {
        let temp_dir = create_test_directory();
        let options = ListingOptions {
            recursive: false,
            indent_level: 0,
        };

        assert!(list_directory(temp_dir.path(), &options).is_ok());
    }

    #[test]
    fn test_directory_not_exists() {
        let non_existent_path = PathBuf::from("/path/that/does/not/exist");
        let options = ListingOptions {
            recursive: false,
            indent_level: 0,
        };

        assert!(list_directory(&non_existent_path, &options).is_err());
    }

    #[test]
    fn test_recursive_listing() {
        let temp_dir = create_test_directory();
        let options = ListingOptions {
            recursive: true,
            indent_level: 0,
        };

        assert!(list_directory(temp_dir.path(), &options).is_ok());
    }

    #[test]
    fn test_file_sizes() {
        let temp_dir = create_test_directory();
        let file_path = temp_dir.path().join("size_test.txt");
        let data = vec![b'a'; 2048]; // 2KB of data

        let mut file = File::create(&file_path).unwrap();
        file.write_all(&data).unwrap();

        let metadata = fs::metadata(&file_path).unwrap();
        assert_eq!(format_size(metadata.len()), "2.00 KB");
    }

    #[test]
    fn test_modification_time() {
        let temp_dir = create_test_directory();
        let file_path = temp_dir.path().join("time_test.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"Test").unwrap();

        let metadata = fs::metadata(&file_path).unwrap();
        assert!(metadata.modified().is_ok());
    }

    #[test]
    fn test_long_filename_truncation() {
        let temp_dir = create_test_directory();
        let long_filename = "a".repeat(50) + ".txt";
        let file_path = temp_dir.path().join(&long_filename);
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"Test").unwrap();

        let options = ListingOptions {
            recursive: false,
            indent_level: 0,
        };

        assert!(list_directory(temp_dir.path(), &options).is_ok());
    }

    #[test]
    fn test_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let options = ListingOptions {
            recursive: true,
            indent_level: 0,
        };

        assert!(list_directory(temp_dir.path(), &options).is_ok());
    }

    #[test]
    fn test_directory_sorting() {
        let temp_dir = TempDir::new().unwrap();

        // Create files and directories in non-alphabetical order
        fs::create_dir(temp_dir.path().join("zdir")).unwrap();
        fs::create_dir(temp_dir.path().join("adir")).unwrap();
        File::create(temp_dir.path().join("c.txt")).unwrap();
        File::create(temp_dir.path().join("b.txt")).unwrap();

        let options = ListingOptions {
            recursive: false,
            indent_level: 0,
        };

        assert!(list_directory(temp_dir.path(), &options).is_ok());
        // Visual inspection would show directories first, then files, both in alphabetical order
    }
}