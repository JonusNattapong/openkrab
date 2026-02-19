use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: Value, // JSON Schema
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: String, // JSON string
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOutput {
    pub call_id: String,
    pub output: String,
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn definition(&self) -> ToolDefinition;
    async fn call(&self, arguments: &str) -> Result<String>;
}

pub struct SearchMemoryTool {
    manager: std::sync::Arc<crate::memory::MemoryManager>,
}

impl SearchMemoryTool {
    pub fn new(manager: std::sync::Arc<crate::memory::MemoryManager>) -> Self {
        Self { manager }
    }
}

#[async_trait]
impl Tool for SearchMemoryTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "search_memory".to_string(),
            description: "Search in the local memory/knowledge base for relevant information."
                .to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The search query to look for in memory."
                    }
                },
                "required": ["query"]
            }),
        }
    }

    async fn call(&self, arguments: &str) -> Result<String> {
        let args: Value = serde_json::from_str(arguments)?;
        let query = args["query"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing query argument"))?;

        let results = self
            .manager
            .search_hybrid(query, Default::default())
            .await?;
        if results.is_empty() {
            return Ok("No relevant information found in memory.".to_string());
        }

        let mut out = "Found the following information in memory:\n\n".to_string();
        for res in results.iter().take(5) {
            out.push_str(&format!(
                "--- Source: {} (Score: {:.2}) ---\n{}\n\n",
                res.path, res.score, res.text
            ));
        }
        Ok(out)
    }
}

pub struct ReadFileTool {
    workspace_root: std::path::PathBuf,
}

impl ReadFileTool {
    pub fn new(root: std::path::PathBuf) -> Self {
        Self {
            workspace_root: root,
        }
    }
}

#[async_trait]
impl Tool for ReadFileTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "read_file".to_string(),
            description: "Read the content of a file within the workspace.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Relative path to the file from the workspace root."
                    }
                },
                "required": ["path"]
            }),
        }
    }

    async fn call(&self, arguments: &str) -> Result<String> {
        let args: Value = serde_json::from_str(arguments)?;
        let rel_path = args["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing path argument"))?;

        let full_path = self.workspace_root.join(rel_path);

        // Security check: ensure path is within workspace
        if !full_path.starts_with(&self.workspace_root) {
            return Err(anyhow::anyhow!(
                "Access denied: Path is outside of workspace."
            ));
        }

        if !full_path.exists() {
            return Ok(format!("File not found: {}", rel_path));
        }

        let content = tokio::fs::read_to_string(full_path).await?;
        Ok(content)
    }
}

pub struct ListFilesTool {
    workspace_root: std::path::PathBuf,
}

impl ListFilesTool {
    pub fn new(root: std::path::PathBuf) -> Self {
        Self {
            workspace_root: root,
        }
    }
}

#[async_trait]
impl Tool for ListFilesTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "list_files".to_string(),
            description: "List files in a directory within the workspace.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Relative path to the directory. Use '.' for root."
                    }
                },
                "required": ["path"]
            }),
        }
    }

    async fn call(&self, arguments: &str) -> Result<String> {
        let args: Value = serde_json::from_str(arguments)?;
        let rel_path = args["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing path argument"))?;

        let full_path = self.workspace_root.join(rel_path);

        if !full_path.starts_with(&self.workspace_root) {
            return Err(anyhow::anyhow!(
                "Access denied: Path is outside of workspace."
            ));
        }

        if !full_path.exists() || !full_path.is_dir() {
            return Ok(format!("Directory not found: {}", rel_path));
        }

        let mut entries = tokio::fs::read_dir(full_path).await?;
        let mut files = Vec::new();
        while let Some(entry) = entries.next_entry().await? {
            let name = entry.file_name().to_string_lossy().to_string();
            let is_dir = entry.file_type().await?.is_dir();
            files.push(format!("{}{}", name, if is_dir { "/" } else { "" }));
        }

        Ok(files.join("\n"))
    }
}

pub struct WriteFileTool {
    workspace_root: std::path::PathBuf,
}

impl WriteFileTool {
    pub fn new(root: std::path::PathBuf) -> Self {
        Self {
            workspace_root: root,
        }
    }
}

#[async_trait]
impl Tool for WriteFileTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "write_file".to_string(),
            description: "Write content to a file within the workspace. Overwrites if exists."
                .to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Relative path to the file from the workspace root."
                    },
                    "content": {
                        "type": "string",
                        "description": "The content to write to the file."
                    }
                },
                "required": ["path", "content"]
            }),
        }
    }

    async fn call(&self, arguments: &str) -> Result<String> {
        let args: Value = serde_json::from_str(arguments)?;
        let rel_path = args["path"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing path argument"))?;
        let content = args["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing content argument"))?;

        let full_path = self.workspace_root.join(rel_path);

        if !full_path.starts_with(&self.workspace_root) {
            return Err(anyhow::anyhow!(
                "Access denied: Path is outside of workspace."
            ));
        }

        // Ensure parent directory exists
        if let Some(parent) = full_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        tokio::fs::write(&full_path, content).await?;
        Ok(format!("Successfully wrote to {}", rel_path))
    }
}

pub struct RememberTool {
    manager: std::sync::Arc<crate::memory::MemoryManager>,
    workspace_root: std::path::PathBuf,
}

impl RememberTool {
    pub fn new(
        manager: std::sync::Arc<crate::memory::MemoryManager>,
        root: std::path::PathBuf,
    ) -> Self {
        Self {
            manager,
            workspace_root: root,
        }
    }
}

#[async_trait]
impl Tool for RememberTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "remember".to_string(),
            description: "Save important information or notes to the agent's long-term memory."
                .to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "note_name": {
                        "type": "string",
                        "description": "A short, descriptive name for the note (e.g. 'project_goals')."
                    },
                    "content": {
                        "type": "string",
                        "description": "The detailed information to remember."
                    }
                },
                "required": ["note_name", "content"]
            }),
        }
    }

    async fn call(&self, arguments: &str) -> Result<String> {
        let args: Value = serde_json::from_str(arguments)?;
        let name = args["note_name"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing note_name argument"))?;
        let content = args["content"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing content argument"))?;

        // Sanitize name for filename
        let safe_name = name
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
            .collect::<String>();
        let rel_path = format!("memory/agent_notes/{}.md", safe_name);
        let full_path = self.workspace_root.join(&rel_path);

        // Ensure parent directory exists
        if let Some(parent) = full_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let md_content = format!(
            "# Agent Note: {}\n\n{}\n\n*Saved at: {}*",
            name,
            content,
            chrono::Local::now()
        );
        tokio::fs::write(&full_path, md_content).await?;

        // Re-index this specific file immediately
        self.manager
            .index_file(&self.workspace_root, &full_path)
            .await?;

        Ok(format!(
            "I have recorded this note as '{}'. It is now part of my long-term memory.",
            name
        ))
    }
}

pub struct ExecCommandTool {
    workspace_root: std::path::PathBuf,
    require_approval: bool,
}

impl ExecCommandTool {
    pub fn new(root: std::path::PathBuf, require_approval: bool) -> Self {
        Self {
            workspace_root: root,
            require_approval,
        }
    }
}

#[async_trait]
impl Tool for ExecCommandTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "exec_command".to_string(),
            description: "Execute a shell command within the workspace root. DANGEROUS: Requires user approval if security is enabled.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "The command to execute (e.g. 'cargo test')."
                    }
                },
                "required": ["command"]
            }),
        }
    }

    async fn call(&self, arguments: &str) -> Result<String> {
        let args: Value = serde_json::from_str(arguments)?;
        let command = args["command"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing command argument"))?;

        if self.require_approval {
            println!("\n⚠️  [SECURITY] Agent wishes to execute: {}", command);
            print!("👉 Approve execution? (y/N): ");
            use std::io::{BufRead, Write};
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().lock().read_line(&mut input)?;
            let input = input.trim().to_lowercase();

            if input != "y" && input != "yes" {
                return Ok("Execution REJECTED by user.".to_string());
            }
        }

        println!("Executing: {}", command);

        // Simple shell execution
        let output = if cfg!(target_os = "windows") {
            std::process::Command::new("powershell")
                .args(["-Command", command])
                .current_dir(&self.workspace_root)
                .output()?
        } else {
            std::process::Command::new("sh")
                .arg("-c")
                .arg(command)
                .current_dir(&self.workspace_root)
                .output()?
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        let mut result = String::new();
        if !stdout.is_empty() {
            result.push_str("STDOUT:\n");
            result.push_str(&stdout);
        }
        if !stderr.is_empty() {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str("STDERR:\n");
            result.push_str(&stderr);
        }
        if result.is_empty() {
            result = if output.status.success() {
                "Success (no output)".to_string()
            } else {
                format!("Failed with status: {}", output.status)
            };
        }

        Ok(result)
    }
}

pub struct TaskTool {
    workspace_root: std::path::PathBuf,
}

impl TaskTool {
    pub fn new(root: std::path::PathBuf) -> Self {
        Self {
            workspace_root: root,
        }
    }

    async fn load_tasks(&self) -> Result<Vec<String>> {
        let task_file = self.workspace_root.join("memory/tasks.md");
        if !task_file.exists() {
            return Ok(Vec::new());
        }
        let content = tokio::fs::read_to_string(&task_file).await?;
        let tasks = content
            .lines()
            .filter(|l| l.starts_with("- [ ] "))
            .map(|l| l.trim_start_matches("- [ ] ").to_string())
            .collect();
        Ok(tasks)
    }

    async fn save_tasks(&self, tasks: &[String]) -> Result<()> {
        let task_file = self.workspace_root.join("memory/tasks.md");
        if let Some(parent) = task_file.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        let content = tasks
            .iter()
            .map(|t| format!("- [ ] {}", t))
            .collect::<Vec<_>>()
            .join("\n");
        tokio::fs::write(task_file, content).await?;
        Ok(())
    }
}

#[async_trait]
impl Tool for TaskTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "manage_tasks".to_string(),
            description: "Manage a todo list of tasks. Can add, list, or remove tasks.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["add", "list", "remove"],
                        "description": "The action to perform."
                    },
                    "task": {
                        "type": "string",
                        "description": "The task description (required for add/remove)."
                    }
                },
                "required": ["action"]
            }),
        }
    }

    async fn call(&self, arguments: &str) -> Result<String> {
        let args: Value = serde_json::from_str(arguments)?;
        let action = args["action"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing action argument"))?;

        // Use self.load_tasks() and self.save_tasks() here.
        // But since they are async and need &self, we just inline or call them.
        // Wait, TaskTool struct implementation is separate from Tool trait impl.

        let task_file = self.workspace_root.join("memory/tasks.md");

        match action {
            "add" => {
                let task = args["task"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing task argument for add"))?;
                let mut tasks = self.load_tasks().await?;
                tasks.push(task.to_string());
                self.save_tasks(&tasks).await?;
                Ok(format!("Added task: {}", task))
            }
            "list" => {
                let tasks = self.load_tasks().await?;
                if tasks.is_empty() {
                    Ok("No pending tasks.".to_string())
                } else {
                    Ok(tasks
                        .iter()
                        .enumerate()
                        .map(|(i, t)| format!("{}. {}", i + 1, t))
                        .collect::<Vec<_>>()
                        .join("\n"))
                }
            }
            "remove" => {
                let task = args["task"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing task argument for remove"))?;
                let mut tasks = self.load_tasks().await?;
                if let Some(idx) = tasks.iter().position(|t| t == task) {
                    tasks.remove(idx);
                    self.save_tasks(&tasks).await?;
                    Ok(format!("Removed task: {}", task))
                } else {
                    Ok(format!("Task not found: {}", task))
                }
            }
            _ => Err(anyhow::anyhow!("Unknown action: {}", action)),
        }
    }
}

pub struct SpeakTool;

impl SpeakTool {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Tool for SpeakTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "speak".to_string(),
            description: "Speak the text using text-to-speech (TTS).".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "text": {
                        "type": "string",
                        "description": "The text to speak."
                    }
                },
                "required": ["text"]
            }),
        }
    }

    async fn call(&self, arguments: &str) -> Result<String> {
        let args: Value = serde_json::from_str(arguments)?;
        let text = args["text"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing text argument"))?;

        let script = format!(
            "Add-Type -AssemblyName System.Speech; \
            $speak = New-Object System.Speech.Synthesis.SpeechSynthesizer; \
            $speak.Speak('{}');",
            text.replace("'", "''") // Escape single quotes
        );

        #[cfg(target_os = "windows")]
        {
            let output = std::process::Command::new("powershell")
                .args(["-Command", &script])
                .output()?;

            if output.status.success() {
                Ok(format!("Spoken: \"{}\"", text))
            } else {
                Err(anyhow::anyhow!(
                    "TTS Failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ))
            }
        }

        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("say").arg(text).output()?;
            Ok(format!("Spoken: \"{}\"", text))
        }

        #[cfg(target_os = "linux")]
        {
            Ok("TTS not fully implemented for Linux yet.".to_string())
        }

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            Ok("TTS only supported on Windows/macOS/Linux.".to_string())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScheduledTask {
    id: String,
    cron: String,
    description: String,
    enabled: bool,
}

pub struct ScheduleTool {
    workspace_root: std::path::PathBuf,
}

impl ScheduleTool {
    pub fn new(root: std::path::PathBuf) -> Self {
        Self {
            workspace_root: root,
        }
    }

    async fn load_schedule(&self) -> Result<Vec<ScheduledTask>> {
        let file = self.workspace_root.join("memory/schedule.json");
        if !file.exists() {
            return Ok(Vec::new());
        }
        let content = tokio::fs::read_to_string(&file).await?;
        let tasks: Vec<ScheduledTask> = serde_json::from_str(&content).unwrap_or_default();
        Ok(tasks)
    }

    async fn save_schedule(&self, tasks: &[ScheduledTask]) -> Result<()> {
        let file = self.workspace_root.join("memory/schedule.json");
        if let Some(parent) = file.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        let content = serde_json::to_string_pretty(tasks)?;
        tokio::fs::write(file, content).await?;
        Ok(())
    }
}

#[async_trait]
impl Tool for ScheduleTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "manage_schedule".to_string(),
            description: "Manage scheduled cron jobs for the agent.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["add", "list", "remove"],
                        "description": "Action to perform."
                    },
                    "cron": {
                        "type": "string",
                        "description": "Cron expression (e.g. '0 9 * * *' for daily at 9am)."
                    },
                    "description": {
                        "type": "string",
                        "description": "Description of the task to perform."
                    },
                    "id": {
                        "type": "string",
                        "description": "Task ID (for removal)."
                    }
                },
                "required": ["action"]
            }),
        }
    }

    async fn call(&self, arguments: &str) -> Result<String> {
        let args: Value = serde_json::from_str(arguments)?;
        let action = args["action"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing action argument"))?;

        match action {
            "add" => {
                let cron = args["cron"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing cron argument"))?;
                let description = args["description"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing description argument"))?;
                let id = uuid::Uuid::new_v4().to_string();

                let mut tasks = self.load_schedule().await?;
                tasks.push(ScheduledTask {
                    id: id.clone(),
                    cron: cron.to_string(),
                    description: description.to_string(),
                    enabled: true,
                });
                self.save_schedule(&tasks).await?;
                Ok(format!("Scheduled task '{}' with ID: {}", description, id))
            }
            "list" => {
                let tasks = self.load_schedule().await?;
                if tasks.is_empty() {
                    Ok("No scheduled tasks.".to_string())
                } else {
                    let list = tasks
                        .iter()
                        .map(|t| {
                            format!(
                                "[{}] {} - {} ({})",
                                if t.enabled { "ON" } else { "OFF" },
                                t.cron,
                                t.description,
                                t.id
                            )
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    Ok(list)
                }
            }
            "remove" => {
                let id = args["id"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("Missing id argument"))?;
                let mut tasks = self.load_schedule().await?;
                let len_before = tasks.len();
                tasks.retain(|t| t.id != id);
                if tasks.len() < len_before {
                    self.save_schedule(&tasks).await?;
                    Ok(format!("Removed task with ID: {}", id))
                } else {
                    Ok(format!("Task not found with ID: {}", id))
                }
            }
            _ => Err(anyhow::anyhow!("Unknown action: {}", action)),
        }
    }
}

pub struct BrowserTool;

impl BrowserTool {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Tool for BrowserTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "browse_url".to_string(),
            description: "Fetch and read the content of a URL as markdown text.".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "The URL to visit."
                    }
                },
                "required": ["url"]
            }),
        }
    }

    async fn call(&self, arguments: &str) -> Result<String> {
        let args: Value = serde_json::from_str(arguments)?;
        let url = args["url"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing url argument"))?;

        let client = reqwest::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .build()?;

        let res = client.get(url).send().await?;
        if !res.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to fetch URL: Status {}",
                res.status()
            ));
        }

        let html = res.text().await?;
        let md = html2md::parse_html(&html);

        // Truncate if too long to save context
        if md.len() > 20000 {
            Ok(format!("(Content truncated) ... \n{}", &md[..20000]))
        } else {
            Ok(md)
        }
    }
}

pub struct CodeInterpreterTool {
    workspace_root: std::path::PathBuf,
}

impl CodeInterpreterTool {
    pub fn new(root: std::path::PathBuf) -> Self {
        Self {
            workspace_root: root,
        }
    }
}

#[async_trait]
impl Tool for CodeInterpreterTool {
    fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: "code_interpreter".to_string(),
            description: "Execute Python or JavaScript code in a sandboxed Docker container."
                .to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "language": {
                        "type": "string",
                        "enum": ["python", "javascript"],
                        "description": "Programming language ('python' or 'javascript')."
                    },
                    "code": {
                        "type": "string",
                        "description": "The source code to execute."
                    }
                },
                "required": ["language", "code"]
            }),
        }
    }

    async fn call(&self, arguments: &str) -> Result<String> {
        let args: Value = serde_json::from_str(arguments)?;
        let language = args["language"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing language argument"))?;
        let code = args["code"]
            .as_str()
            .ok_or_else(|| anyhow::anyhow!("Missing code argument"))?;

        let sandbox_dir = self.workspace_root.join(".sandbox");
        if !sandbox_dir.exists() {
            tokio::fs::create_dir_all(&sandbox_dir).await?;
        }

        let (image, filename, cmd) = match language {
            "python" | "py" => ("python:3.9-slim", "script.py", "python"),
            "javascript" | "js" => ("node:18-alpine", "script.js", "node"),
            _ => return Err(anyhow::anyhow!("Unsupported language: {}", language)),
        };

        let script_path = sandbox_dir.join(filename);
        tokio::fs::write(&script_path, code).await?;

        // Absolute path for Docker volume mount
        let abs_path = std::fs::canonicalize(&sandbox_dir)?;
        let mut abs_path_str = abs_path.to_string_lossy().to_string();

        // Remove extended length path prefix on Windows if present
        if cfg!(target_os = "windows") && abs_path_str.starts_with(r"\\?\") {
            abs_path_str = abs_path_str[4..].to_string();
        }

        let output = std::process::Command::new("docker")
            .args([
                "run",
                "--rm",
                "-v",
                &format!("{}:/app", abs_path_str),
                "-w",
                "/app",
                image,
                cmd,
                filename,
            ])
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if output.status.success() {
            Ok(format!("Output:\n{}", stdout))
        } else {
            Ok(format!(
                "Execution Failed:\nSTDOUT:\n{}\nSTDERR:\n{}",
                stdout, stderr
            ))
        }
    }
}
