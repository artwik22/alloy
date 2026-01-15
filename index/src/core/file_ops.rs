use std::fs;
use std::io;
use std::path::Path;

pub struct FileOperations;

impl FileOperations {
    pub fn copy_file(source: &Path, destination: &Path) -> io::Result<()> {
        if source.is_dir() {
            Self::copy_dir_recursive(source, destination)
        } else {
            fs::copy(source, destination)?;
            Ok(())
        }
    }

    fn copy_dir_recursive(source: &Path, destination: &Path) -> io::Result<()> {
        fs::create_dir_all(destination)?;

        for entry in fs::read_dir(source)? {
            let entry = entry?;
            let source_path = entry.path();
            let dest_path = destination.join(entry.file_name());

            if source_path.is_dir() {
                Self::copy_dir_recursive(&source_path, &dest_path)?;
            } else {
                fs::copy(&source_path, &dest_path)?;
            }
        }

        Ok(())
    }

    pub fn move_file(source: &Path, destination: &Path) -> io::Result<()> {
        // Try rename first (faster for same filesystem)
        if fs::rename(source, destination).is_ok() {
            return Ok(());
        }

        // Fallback to copy + delete
        Self::copy_file(source, destination)?;
        if source.is_dir() {
            fs::remove_dir_all(source)?;
        } else {
            fs::remove_file(source)?;
        }

        Ok(())
    }

    pub fn delete(path: &Path) -> Result<(), trash::Error> {
        trash::delete(path)
    }

    pub fn rename(path: &Path, new_name: &str) -> io::Result<()> {
        let parent = path.parent().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "No parent directory")
        })?;
        let new_path = parent.join(new_name);
        fs::rename(path, new_path)
    }

    pub fn create_directory(path: &Path) -> io::Result<()> {
        fs::create_dir(path)
    }

    pub fn create_file(path: &Path) -> io::Result<()> {
        fs::File::create(path)?;
        Ok(())
    }
}
