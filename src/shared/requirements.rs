//! Requirements checking utilities for feature dependencies

use std::collections::HashMap;

/// Requirement check result
#[derive(Debug, Clone, PartialEq)]
pub enum RequirementResult {
    Met,
    NotMet(String),
}

/// A single requirement check
pub type RequirementCheck = Box<dyn Fn() -> RequirementResult + Send + Sync>;

/// Requirements checker
pub struct RequirementsChecker {
    checks: HashMap<String, RequirementCheck>,
}

impl Default for RequirementsChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl RequirementsChecker {
    /// Create new requirements checker
    pub fn new() -> Self {
        Self {
            checks: HashMap::new(),
        }
    }

    /// Register a requirement check
    pub fn register<F>(&mut self, name: &str, check: F)
    where
        F: Fn() -> RequirementResult + Send + Sync + 'static,
    {
        self.checks.insert(name.to_string(), Box::new(check));
    }

    /// Check a specific requirement
    pub fn check(&self, name: &str) -> RequirementResult {
        match self.checks.get(name) {
            Some(check) => check(),
            None => RequirementResult::NotMet(format!("Unknown requirement: {}", name)),
        }
    }

    /// Check all requirements
    pub fn check_all(&self) -> HashMap<String, RequirementResult> {
        self.checks
            .iter()
            .map(|(name, check)| (name.clone(), check()))
            .collect()
    }

    /// Get all unmet requirements
    pub fn unmet_requirements(&self) -> Vec<(String, String)> {
        self.check_all()
            .into_iter()
            .filter_map(|(name, result)| match result {
                RequirementResult::NotMet(reason) => Some((name, reason)),
                RequirementResult::Met => None,
            })
            .collect()
    }

    /// Check if all requirements are met
    pub fn all_met(&self) -> bool {
        self.unmet_requirements().is_empty()
    }
}

/// Check if a binary is available in PATH
pub fn check_binary(name: &str) -> RequirementResult {
    if which::which(name).is_ok() {
        RequirementResult::Met
    } else {
        RequirementResult::NotMet(format!("Binary '{}' not found in PATH", name))
    }
}

/// Check if an environment variable is set
pub fn check_env_var(name: &str) -> RequirementResult {
    if std::env::var(name).is_ok() {
        RequirementResult::Met
    } else {
        RequirementResult::NotMet(format!("Environment variable '{}' not set", name))
    }
}

/// Check if a file exists
pub fn check_file(path: &str) -> RequirementResult {
    if std::path::Path::new(path).exists() {
        RequirementResult::Met
    } else {
        RequirementResult::NotMet(format!("File '{}' not found", path))
    }
}

/// Check if a directory exists
pub fn check_directory(path: &str) -> RequirementResult {
    let path = std::path::Path::new(path);
    if path.is_dir() {
        RequirementResult::Met
    } else {
        RequirementResult::NotMet(format!("Directory '{}' not found", path.display()))
    }
}

/// Requirement set for common features
pub struct CommonRequirements;

impl CommonRequirements {
    /// Check for Docker availability
    pub fn docker() -> RequirementResult {
        check_binary("docker")
    }

    /// Check for Git availability
    pub fn git() -> RequirementResult {
        check_binary("git")
    }

    /// Check for Node.js availability
    pub fn node() -> RequirementResult {
        check_binary("node")
    }

    /// Check for Python availability
    pub fn python() -> RequirementResult {
        check_binary("python")
    }

    /// Check for curl availability
    pub fn curl() -> RequirementResult {
        check_binary("curl")
    }

    /// Check for OpenAI API key
    pub fn openai_key() -> RequirementResult {
        check_env_var("OPENAI_API_KEY")
    }
}

/// Build a requirements checker with common checks
pub fn common_requirements_checker() -> RequirementsChecker {
    let mut checker = RequirementsChecker::new();

    checker.register("docker", || CommonRequirements::docker());
    checker.register("git", || CommonRequirements::git());
    checker.register("node", || CommonRequirements::node());
    checker.register("python", || CommonRequirements::python());
    checker.register("curl", || CommonRequirements::curl());

    checker
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_requirements_checker() {
        let mut checker = RequirementsChecker::new();

        checker.register("always_met", || RequirementResult::Met);
        checker.register("never_met", || {
            RequirementResult::NotMet("test reason".to_string())
        });

        assert_eq!(checker.check("always_met"), RequirementResult::Met);
        assert!(matches!(
            checker.check("never_met"),
            RequirementResult::NotMet(_)
        ));
        assert!(!checker.all_met());
    }

    #[test]
    fn test_check_file() {
        // This file should exist
        assert_eq!(
            check_file("Cargo.toml"),
            RequirementResult::Met,
            "Cargo.toml should exist"
        );

        // This file should not exist
        assert!(
            matches!(
                check_file("nonexistent_file_xyz.abc"),
                RequirementResult::NotMet(_)
            ),
            "nonexistent file should return NotMet"
        );
    }

    #[test]
    fn test_check_directory() {
        // This directory should exist
        assert_eq!(
            check_directory("src"),
            RequirementResult::Met,
            "src directory should exist"
        );

        // This directory should not exist
        assert!(
            matches!(
                check_directory("nonexistent_dir_xyz"),
                RequirementResult::NotMet(_)
            ),
            "nonexistent directory should return NotMet"
        );
    }
}
