//! PID alive checking utilities

use std::path::Path;

/// Check if a process with the given PID is alive
pub fn is_pid_alive(pid: u32) -> bool {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("tasklist")
            .args(["/FI", &format!("PID eq {}", pid), "/NH"])
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).contains(&pid.to_string()))
            .unwrap_or(false)
    }

    #[cfg(not(target_os = "windows"))]
    {
        // On Unix, check if /proc/PID exists or send signal 0
        Path::new(&format!("/proc/{}", pid)).exists()
    }
}

/// Check if a process is alive by PID file
pub fn is_pid_file_alive(pid_file: &Path) -> bool {
    match std::fs::read_to_string(pid_file) {
        Ok(content) => content
            .trim()
            .parse::<u32>()
            .map(is_pid_alive)
            .unwrap_or(false),
        Err(_) => false,
    }
}

/// Get PID from file if it exists and is valid
pub fn get_pid_from_file(pid_file: &Path) -> Option<u32> {
    std::fs::read_to_string(pid_file)
        .ok()?
        .trim()
        .parse::<u32>()
        .ok()
}

/// Write PID to file
pub fn write_pid_file(pid_file: &Path) -> std::io::Result<()> {
    let pid = std::process::id();
    std::fs::write(pid_file, pid.to_string())
}

/// Remove PID file if the process is not alive
pub fn cleanup_pid_file(pid_file: &Path) -> std::io::Result<bool> {
    if !is_pid_file_alive(pid_file) {
        std::fs::remove_file(pid_file)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

/// PID file manager for daemon processes
pub struct PidFile {
    path: std::path::PathBuf,
}

impl PidFile {
    /// Create new PID file manager
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    /// Check if managed process is alive
    pub fn is_alive(&self) -> bool {
        is_pid_file_alive(&self.path)
    }

    /// Get PID from file
    pub fn pid(&self) -> Option<u32> {
        get_pid_from_file(&self.path)
    }

    /// Write current PID to file
    pub fn write(&self) -> std::io::Result<()> {
        write_pid_file(&self.path)
    }

    /// Remove PID file
    pub fn remove(&self) -> std::io::Result<()> {
        std::fs::remove_file(&self.path)
    }

    /// Cleanup if process is dead
    pub fn cleanup(&self) -> std::io::Result<bool> {
        cleanup_pid_file(&self.path)
    }

    /// Get path
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for PidFile {
    fn drop(&mut self) {
        // Only remove if it's our PID
        if let Some(pid) = self.pid() {
            if pid == std::process::id() {
                let _ = self.remove();
            }
        }
    }
}

/// Find processes by name (best effort)
pub fn find_processes_by_name(name: &str) -> Vec<u32> {
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        match Command::new("tasklist")
            .args(["/FO", "CSV", "/NH"])
            .output()
        {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout
                    .lines()
                    .filter(|line| line.to_lowercase().contains(&name.to_lowercase()))
                    .filter_map(|line| {
                        line.split(',')
                            .nth(1)
                            .and_then(|s| s.trim_matches('"').parse::<u32>().ok())
                    })
                    .collect()
            }
            Err(_) => Vec::new(),
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        use std::process::Command;
        match Command::new("pgrep").args(["-f", name]).output() {
            Ok(output) => String::from_utf8_lossy(&output.stdout)
                .lines()
                .filter_map(|line| line.trim().parse::<u32>().ok())
                .collect(),
            Err(_) => Vec::new(),
        }
    }
}

/// Kill a process by PID (graceful then force)
pub fn kill_process(pid: u32, graceful_timeout_ms: u64) -> bool {
    #[cfg(not(target_os = "windows"))]
    {
        // Try graceful termination first
        unsafe {
            libc::kill(pid as i32, libc::SIGTERM);
        }

        // Wait for graceful shutdown
        std::thread::sleep(std::time::Duration::from_millis(graceful_timeout_ms));

        // Check if still alive
        if is_pid_alive(pid) {
            // Force kill
            unsafe {
                libc::kill(pid as i32, libc::SIGKILL);
            }
        }

        !is_pid_alive(pid)
    }

    #[cfg(target_os = "windows")]
    {
        use std::process::Command;

        // Try graceful termination
        let _ = Command::new("taskkill")
            .args(["/PID", &pid.to_string()])
            .output();

        std::thread::sleep(std::time::Duration::from_millis(graceful_timeout_ms));

        // Force kill if still alive
        if is_pid_alive(pid) {
            let _ = Command::new("taskkill")
                .args(["/F", "/PID", &pid.to_string()])
                .output();
        }

        !is_pid_alive(pid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_is_pid_alive_current() {
        // Current process should be alive
        assert!(is_pid_alive(std::process::id()));
    }

    #[test]
    fn test_is_pid_alive_invalid() {
        // PID 0 and very high PIDs should not be alive
        assert!(!is_pid_alive(0));
        assert!(!is_pid_alive(999_999));
    }

    #[test]
    fn test_pid_file() {
        let temp_dir = std::env::temp_dir();
        let pid_file = temp_dir.join("test_pid_file.tmp");

        // Write current PID
        write_pid_file(&pid_file).unwrap();
        assert!(pid_file.exists());

        // Should be alive
        assert!(is_pid_file_alive(&pid_file));

        // Get PID
        let pid = get_pid_from_file(&pid_file).unwrap();
        assert_eq!(pid, std::process::id());

        // Cleanup
        std::fs::remove_file(&pid_file).unwrap();
    }

    #[test]
    fn test_pid_file_not_alive() {
        let temp_dir = std::env::temp_dir();
        let pid_file = temp_dir.join("test_pid_file_dead.tmp");

        // Write a PID that doesn't exist
        let mut file = std::fs::File::create(&pid_file).unwrap();
        file.write_all(b"999999").unwrap();
        drop(file);

        // Should not be alive
        assert!(!is_pid_file_alive(&pid_file));

        // Cleanup
        std::fs::remove_file(&pid_file).unwrap();
    }

    #[test]
    fn test_pid_file_manager() {
        let temp_dir = std::env::temp_dir();
        let pid_file = PidFile::new(temp_dir.join("test_pid_mgr.tmp"));

        // Write PID
        pid_file.write().unwrap();
        assert!(pid_file.is_alive());
        assert_eq!(pid_file.pid(), Some(std::process::id()));

        // Cleanup
        pid_file.remove().unwrap();
    }
}
