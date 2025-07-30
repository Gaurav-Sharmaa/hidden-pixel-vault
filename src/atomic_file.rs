use crate::Result;
use std::fs;
use std::path::{Path, PathBuf};

pub struct AtomicFileHandler {
    target_path: PathBuf,
    temp_path: PathBuf,
    backup_path: PathBuf,
}

impl AtomicFileHandler {
    /// Creates a new AtomicFileHandler for the given file path
    pub fn new(file_path: &str) -> Result<Self> {
        let target = PathBuf::from(file_path);

        // Validate target file exists
        if !target.exists() {
            return Err(format!("File does not exist: {}", file_path).into());
        }

        // Generate temp and backup paths
        let temp = Self::generate_temp_path(&target)?;
        let backup = Self::generate_backup_path(&target)?;

        Ok(AtomicFileHandler {
            target_path: target,
            temp_path: temp,
            backup_path: backup,
        })
    }

    /// Generates temporary file path: file.png -> file.png.tmp
    fn generate_temp_path(target_path: &Path) -> Result<PathBuf> {
        let mut temp = target_path.to_path_buf();
        temp.set_extension(format!(
            "{}.tmp",
            target_path
                .extension()
                .ok_or("File must have an extension")?
                .to_string_lossy()
        ));
        Ok(temp)
    }

    /// Generates backup file path: file.png -> file.png.backup
    fn generate_backup_path(target_path: &Path) -> Result<PathBuf> {
        let mut backup = target_path.to_path_buf();
        backup.set_extension(format!(
            "{}.backup",
            target_path
                .extension()
                .ok_or("File must have an extension")?
                .to_string_lossy()
        ));
        Ok(backup)
    }

    /// Read the target file for operations that don't modify it
    pub fn read_file(&self) -> Result<Vec<u8>> {
        fs::read(&self.target_path).map_err(|e| {
            format!(
                "Failed to read file '{}': {}",
                self.target_path.display(),
                e
            )
            .into()
        })
    }

    /// Create a backup of the original file before modification
    pub fn create_backup(&self) -> Result<()> {
        println!("🛡️  Created Backup: {}", self.backup_path.display());

        fs::copy(&self.target_path, &self.backup_path)
            .map_err(|e| format!("Failed to create backup: {}", e))?;

        Ok(())
    }

    /// Create a backup silently (no output message)
    pub fn create_backup_silent(&self) -> Result<()> {
        fs::copy(&self.target_path, &self.backup_path)
            .map_err(|e| format!("Failed to create backup: {}", e))?;
        Ok(())
    }

    /// Start atomic modification: creates temp file with current content
    pub fn begin_atomic_write(&self) -> Result<Vec<u8>> {
        // Create backup first
        self.create_backup()?;

        // Read current content
        let content = self.read_file()?;

        // Create temp file with current content
        fs::write(&self.temp_path, &content)
            .map_err(|e| format!("Failed to create temporary file: {}", e))?;

        Ok(content)
    }

    /// Start atomic modification silently: creates temp file with current content
    pub fn begin_atomic_write_silent(&self) -> Result<Vec<u8>> {
        // Create backup silently
        self.create_backup_silent()?;

        // Read current content
        let content = self.read_file()?;

        // Create temp file with current content
        fs::write(&self.temp_path, &content)
            .map_err(|e| format!("Failed to create temporary file: {}", e))?;

        Ok(content)
    }

    /// Write modified content to temp file
    pub fn write_temp(&self, data: &[u8]) -> Result<()> {
        fs::write(&self.temp_path, data)
            .map_err(|e| format!("Failed to write to temporary file: {}", e).into())
    }

    /// Commit atomic operation: atomically replace target with temp file
    pub fn commit_atomic_write(&self) -> Result<()> {
        // Atomic rename (this is the critical atomic operation)
        fs::rename(&self.temp_path, &self.target_path)
            .map_err(|e| format!("Failed to commit changes: {}", e))?;

        Ok(())
    }

    /// Rollback: restore from backup and cleanup temp files
    pub fn rollback(&self) -> Result<()> {
        // Clean up temp file if it exists
        if self.temp_path.exists() {
            fs::remove_file(&self.temp_path)
                .map_err(|e| format!("Failed to remove temp file during rollback: {}", e))?;
        }

        // Restore from backup if it exists
        if self.backup_path.exists() {
            fs::copy(&self.backup_path, &self.target_path)
                .map_err(|e| format!("Failed to restore from backup: {}", e))?;
        }

        Ok(())
    }

    /// Rollback silently: restore from backup and cleanup temp files without messages
    pub fn rollback_silent(&self) -> Result<()> {
        // Clean up temp file if it exists
        if self.temp_path.exists() {
            fs::remove_file(&self.temp_path)
                .map_err(|e| format!("Failed to remove temp file during rollback: {}", e))?;
        }

        // Restore from backup if it exists
        if self.backup_path.exists() {
            fs::copy(&self.backup_path, &self.target_path)
                .map_err(|e| format!("Failed to restore from backup: {}", e))?;
        }

        Ok(())
    }

    /// Restore original file from backup (user command)
    pub fn restore_original(&self) -> Result<()> {
        if !self.backup_path.exists() {
            return Err("No backup file found. Cannot restore original.".into());
        }

        println!("🔄  Restoring original file from backup...");

        fs::copy(&self.backup_path, &self.target_path)
            .map_err(|e| format!("Failed to restore original file: {}", e))?;

        println!("    Original file restored successfully ");
        println!("    File: {}", self.target_path.display());
        println!("    Restored from: {}", self.backup_path.display());
        Ok(())
    }

    /// Clean up backup and temp files
    pub fn cleanup(&self) -> Result<()> {
        let mut cleaned = Vec::new();

        if self.temp_path.exists() {
            fs::remove_file(&self.temp_path)
                .map_err(|e| format!("Failed to remove temp file: {}", e))?;
            cleaned.push("temp file");
        }

        if self.backup_path.exists() {
            fs::remove_file(&self.backup_path)
                .map_err(|e| format!("Failed to remove backup file: {}", e))?;
            cleaned.push("backup file");
        }

        if !cleaned.is_empty() {
            if cleaned.len() == 1 && cleaned[0] == "backup file" {
                println!(" 🧹  Cleaned up: backup file is removed");
            } else {
                println!(" 🧹  Cleaned up: {}", cleaned.join(" and "));
            }
        } else {
            println!(" ℹ️   No files to clean up");
        }

        Ok(())
    }

    /// Check if backup exists
    pub fn has_backup(&self) -> bool {
        self.backup_path.exists()
    }

    /// Get file paths for display
    pub fn target_path(&self) -> &Path {
        &self.target_path
    }

    pub fn backup_path(&self) -> &Path {
        &self.backup_path
    }
}

// Safe atomic operation wrapper
impl AtomicFileHandler {
    /// Execute a modification operation atomically with auto-rollback on failure
    pub fn atomic_modify<F>(&self, modify_fn: F) -> Result<()>
    where
        F: FnOnce(Vec<u8>) -> Result<Vec<u8>>,
    {
        // Begin atomic operation
        let original_content = self.begin_atomic_write()?;

        // Apply modification
        match modify_fn(original_content) {
            Ok(modified_content) => {
                // Write to temp file
                if let Err(e) = self.write_temp(&modified_content) {
                    self.rollback()?;
                    return Err(e);
                }

                // Commit changes
                if let Err(e) = self.commit_atomic_write() {
                    self.rollback()?;
                    return Err(e);
                }

                Ok(())
            }
            Err(e) => {
                // Modification failed, rollback
                self.rollback()?;
                Err(e)
            }
        }
    }

    /// Execute a modification operation atomically with silent backup and rollback
    pub fn atomic_modify_silent<F>(&self, modify_fn: F) -> Result<()>
    where
        F: FnOnce(Vec<u8>) -> Result<Vec<u8>>,
    {
        // Begin atomic operation silently
        let original_content = self.begin_atomic_write_silent()?;

        // Apply modification
        match modify_fn(original_content) {
            Ok(modified_content) => {
                // Write to temp file
                if let Err(e) = self.write_temp(&modified_content) {
                    self.rollback_silent()?;
                    return Err(e);
                }

                // Commit changes
                if let Err(e) = self.commit_atomic_write() {
                    self.rollback_silent()?;
                    return Err(e);
                }

                Ok(())
            }
            Err(e) => {
                // Modification failed, rollback silently
                self.rollback_silent()?;
                Err(e)
            }
        }
    }
}
