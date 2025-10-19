use std::io::{self, Write};
pub trait Uploader {
    fn upload(data: &[u8], path: Option<&str>) -> io::Result<()>;
}

#[derive(Debug)]
pub struct FileUpload;

impl Uploader for FileUpload {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn file_system_output() {
        let data = "tests".as_bytes();
        let path = "test_file.csv";
        let _ = FileUpload::upload(data, Some(path));
        assert!(std::path::Path::new("test_file.csv").exists());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn file_system_output_temp() {
        let data = "tests".as_bytes();
        let temp_file = "test_output.tmp";
        let _ = FileUpload::upload(data, Some(temp_file)).unwrap();
        let content = fs::read_to_string(temp_file).unwrap();
        assert_eq!(content, "tests");
        let _ = fs::remove_file(temp_file);
    }
    #[test]
    fn io_output_stdout() {
        let data = "TEST STDOUT\n".as_bytes();
        FileUpload::upload(data, None).unwrap();
    }
}
