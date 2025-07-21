

use serde::{Serialize, Deserialize};
use std::fs;
use std::path::{Path, PathBuf};
use crate::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct CfxFile {
    path: String,
    is_dir: bool,
}

#[derive(Debug)]
pub struct CfxSharedVolume {
    base_dir: PathBuf,
}

impl CfxSharedVolume {
    /// Creates a new shared volume rooted at base_dir
    pub fn new(base_dir: &str) -> Self {
        let base = PathBuf::from(base_dir);
        if !base.exists() {
            fs::create_dir_all(&base).expect("Failed to create base_dir—check permissions");
        }
        Self { base_dir: base }
    }

    /// Lists files and directories in parent_dir relative to base_dir
    pub fn ls(&self, parent_dir: &str) -> Result<Vec<CfxFile>, Error> {
        let path = self.base_dir.join(parent_dir);
        let entries = fs::read_dir(&path)
            .map_err(|e| Error::Generic(format!("Failed to read dir {}: {}", path.display(), e)))?
            .filter_map(|entry| {
                let entry = entry.ok()?; // Skip failed entries
                let path = entry.path();
                Some(CfxFile {
                    path: path.strip_prefix(&self.base_dir).unwrap_or(&path)
                        .to_string_lossy().into_owned(),
                    is_dir: path.is_dir(),
                })
            })
            .collect();
        Ok(entries)
    }

    /// Creates a directory (recursive optional)
    pub fn mkdir(&self, dirname: &str, is_recursive: bool) -> Result<(), Error> {
        let path = self.base_dir.join(dirname);
        if is_recursive {
            fs::create_dir_all(&path)
        } else {
            fs::create_dir(&path)
        }.map_err(|e| Error::Generic(format!("Failed to create dir {}: {}", path.display(), e)))?;
        Ok(())
    }

    /// Deletes a file or directory (recursive optional)
    pub fn delete(&self, target: &str, is_recursive: bool) -> Result<(), Error> {
        let path = self.base_dir.join(target);
        if is_recursive && path.is_dir() {
            fs::remove_dir_all(&path)
        } else if path.is_dir() {
            fs::remove_dir(&path)
        } else {
            fs::remove_file(&path)
        }.map_err(|e| Error::Generic(format!("Failed to delete {}: {}", path.display(), e)))?;
        Ok(())
    }

    /// Renames a file or directory
    pub fn rename(&self, source: &str, target: &str) -> Result<(), Error> {
        let source_path = self.base_dir.join(source);
        let target_path = self.base_dir.join(target);
        fs::rename(&source_path, &target_path)
            .map_err(|e| Error::Generic(format!("Failed to rename {} to {}: {}", source_path.display(), target_path.display(), e)))?;
        Ok(())
    }

    /// Copies a file or directory (recursive for dirs)
    pub fn copy(&self, source: &str, target: &str) -> Result<(), Error> {
        let source_path = self.base_dir.join(source);
        let target_path = self.base_dir.join(target);
        if source_path.is_dir() {
            copy_dir(&source_path, &target_path)
        } else {
            fs::copy(&source_path, &target_path).map(|_| ())
        }.map_err(|e| Error::Generic(format!("Failed to copy {} to {}: {}", source_path.display(), target_path.display(), e)))?;
        Ok(())
    }
}

// Helper to recursively copy directories
fn copy_dir(src: &Path, dst: &Path) -> Result<(), std::io::Error> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}


