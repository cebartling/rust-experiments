use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct ServerConfig {
    image_dir: PathBuf,
    max_width: Option<u32>,
    max_height: Option<u32>,
}

impl ServerConfig {
    pub fn new<P: AsRef<Path>>(image_dir: P) -> Self {
        Self {
            image_dir: image_dir.as_ref().to_path_buf(),
            max_width: None,
            max_height: None,
        }
    }

    pub fn with_max_dimensions(mut self, max_width: u32, max_height: u32) -> Self {
        self.max_width = Some(max_width);
        self.max_height = Some(max_height);
        self
    }

    pub fn validate(&self) -> std::io::Result<()> {
        if !self.image_dir.exists() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Image directory does not exist",
            ));
        }
        if !self.image_dir.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Image directory path is not a directory",
            ));
        }
        Ok(())
    }

    pub fn image_dir(&self) -> &Path {
        &self.image_dir
    }

    pub fn max_dimensions(&self) -> (Option<u32>, Option<u32>) {
        (self.max_width, self.max_height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_config_validation_nonexistent_dir() {
        let config = ServerConfig::new("nonexistent_directory");
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_valid_dir() {
        let temp_dir = TempDir::new().unwrap();
        let config = ServerConfig::new(temp_dir.path());
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_max_dimensions() {
        let config = ServerConfig::new("some_dir")
            .with_max_dimensions(100, 200);
        let (max_width, max_height) = config.max_dimensions();
        assert_eq!(max_width, Some(100));
        assert_eq!(max_height, Some(200));
    }
}
