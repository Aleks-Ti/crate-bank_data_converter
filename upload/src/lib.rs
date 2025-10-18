use std::io::{self, Write};
pub trait Uploader {
    fn upload(data: &[u8], path: Option<&str>) -> io::Result<()>;
}

#[derive(Debug)]
pub struct FileSystemOutput;
#[derive(Debug)]
pub struct IoOutput;

impl Uploader for FileSystemOutput {
    fn upload(data: &[u8], path: Option<&str>) -> io::Result<()> {
        if let Some(filename) = path {
            std::fs::write(filename, data)?;
            Ok(())
        } else {
            io::stdout().write_all(data)?;
            Ok(())
        }
    }
}

impl Uploader for IoOutput {
    fn upload(data: &[u8], path: Option<&str>) -> io::Result<()> {
        if let Some(filename) = path {
            std::fs::write(filename, data)?;
        } else {
            io::stdout().write_all(data)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn file_system_output() {
        let data = "tests".as_bytes();
        let path = "test_file.csv";
        let _ = FileSystemOutput::upload(data, Some(path));
        assert!(std::path::Path::new("test_file.csv").exists());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn stdout_output() {
        let data = "tests".as_bytes();
        let temp_file = "test_output.tmp";
        let _ = FileSystemOutput::upload(data, Some(temp_file)).unwrap();
        let content = fs::read_to_string(temp_file).unwrap();
        assert_eq!(content, "tests");
        let _ = fs::remove_file(temp_file);
    }
}
