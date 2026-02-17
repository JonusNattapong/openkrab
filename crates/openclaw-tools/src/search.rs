use super::ToolHandler;
use openclaw_core::{ParameterType, Tool, ToolParameter};
use openclaw_errors::{OpenClawError, Result};
use async_trait::async_trait;
use tokio::fs;
use tracing::debug;
use walkdir::WalkDir;

/// Search tool for finding text in files
pub struct SearchTool;

impl SearchTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ToolHandler for SearchTool {
    fn definition(&self) -> Tool {
        Tool {
            name: "search".to_string(),
            description: "Search for text patterns in files".to_string(),
            parameters: vec![
                ToolParameter {
                    name: "pattern".to_string(),
                    description: "Text pattern or regex to search for".to_string(),
                    param_type: ParameterType::String { enum_values: None },
                    required: true,
                    default: None,
                },
                ToolParameter {
                    name: "path".to_string(),
                    description: "Directory or file to search in".to_string(),
                    param_type: ParameterType::String { enum_values: None },
                    required: false,
                    default: Some(serde_json::json!(".")),
                },
                ToolParameter {
                    name: "file_pattern".to_string(),
                    description: "Glob pattern for files to include (e.g., '*.rs')".to_string(),
                    param_type: ParameterType::String { enum_values: None },
                    required: false,
                    default: None,
                },
                ToolParameter {
                    name: "max_results".to_string(),
                    description: "Maximum number of results to return".to_string(),
                    param_type: ParameterType::Integer,
                    required: false,
                    default: Some(serde_json::json!(50)),
                },
            ],
            returns: openclaw_core::ToolReturn {
                description: "Search results".to_string(),
                return_type: ParameterType::Object {
                    properties: [
                        (
                            "results".to_string(),
                            ParameterType::Array {
                                items: Box::new(ParameterType::Object {
                                    properties: [
                                        ("file".to_string(), ParameterType::String { enum_values: None }),
                                        ("line".to_string(), ParameterType::Integer),
                                        ("content".to_string(), ParameterType::String { enum_values: None }),
                                    ]
                                    .into_iter()
                                    .collect(),
                                }),
                            },
                        ),
                        ("total".to_string(), ParameterType::Integer),
                    ]
                    .into_iter()
                    .collect(),
                },
            },
            dangerous: false,
        }
    }
    
    async fn execute(&self, arguments: serde_json::Value) -> Result<serde_json::Value> {
        let pattern = arguments
            .get("pattern")
            .and_then(|v| v.as_str())
            .ok_or_else(|| OpenClawError::Tool("Missing 'pattern' argument".to_string()))?;
        
        let path = arguments
            .get("path")
            .and_then(|v| v.as_str())
            .unwrap_or(".");
        
        let file_pattern = arguments.get("file_pattern").and_then(|v| v.as_str());
        
        let max_results = arguments
            .get("max_results")
            .and_then(|v| v.as_u64())
            .unwrap_or(50) as usize;
        
        debug!("Searching for '{}' in {}", pattern, path);
        
        let mut results = vec![];
        let path_obj = std::path::Path::new(path);
        
        if path_obj.is_file() {
            // Search single file
            if let Ok(content) = fs::read_to_string(path).await {
                search_in_content(&content, pattern, path, &mut results, max_results);
            }
        } else {
            // Search directory
            for entry in WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                if results.len() >= max_results {
                    break;
                }
                
                let file_path = entry.path();
                
                // Check file pattern
                if let Some(pattern) = file_pattern {
                    if let Some(name) = file_path.file_name().and_then(|n| n.to_str()) {
                        if !glob_match(pattern, name) {
                            continue;
                        }
                    }
                }
                
                // Try to read and search
                if let Ok(content) = fs::read_to_string(file_path).await {
                    let path_str = file_path.to_string_lossy();
                    search_in_content(&content, pattern, &path_str, &mut results, max_results);
                }
            }
        }
        
        let total = results.len();
        
        Ok(serde_json::json!({
            "results": results,
            "total": total,
            "pattern": pattern,
        }))
    }
}

impl Default for SearchTool {
    fn default() -> Self {
        Self::new()
    }
}

fn search_in_content(
    content: &str,
    pattern: &str,
    file_path: &str,
    results: &mut Vec<serde_json::Value>,
    max_results: usize,
) {
    for (line_num, line) in content.lines().enumerate() {
        if results.len() >= max_results {
            break;
        }
        
        if line.contains(pattern) {
            results.push(serde_json::json!({
                "file": file_path,
                "line": line_num + 1,
                "content": line.trim(),
            }));
        }
    }
}

fn glob_match(pattern: &str, name: &str) -> bool {
    // Simple glob matching - could use glob crate for full support
    if pattern.starts_with("*.") {
        let ext = &pattern[1..]; // Remove *
        name.ends_with(ext)
    } else if pattern.contains('*') {
        let parts: Vec<&str> = pattern.split('*').collect();
        let mut remaining = name;
        for (i, part) in parts.iter().enumerate() {
            if part.is_empty() {
                continue;
            }
            if i == 0 && !name.starts_with(part) {
                return false;
            }
            if let Some(pos) = remaining.find(part) {
                remaining = &remaining[pos + part.len()..];
            } else {
                return false;
            }
        }
        true
    } else {
        name == pattern
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_glob_match() {
        assert!(glob_match("*.rs", "test.rs"));
        assert!(!glob_match("*.rs", "test.txt"));
        assert!(glob_match("file*", "file_name"));
        assert!(glob_match("*test*", "my_test_file"));
    }
}
