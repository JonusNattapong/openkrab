//! Skills module - Load and manage skills from SKILL.md files

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub homepage: Option<String>,
    #[serde(default)]
    pub emoji: Option<String>,
    #[serde(default)]
    pub requires: SkillRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SkillRequirements {
    #[serde(default)]
    pub bins: Vec<String>,
    #[serde(default)]
    pub env: Vec<String>,
    #[serde(default)]
    pub os: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillEntry {
    pub name: String,
    pub description: String,
    pub path: PathBuf,
    pub metadata: SkillMetadata,
    pub eligible: bool,
    pub missing: SkillRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillsReport {
    pub skills_dir: String,
    pub bundled_dir: String,
    pub skills: Vec<SkillEntry>,
}

fn parse_frontmatter(content: &str) -> Option<SkillMetadata> {
    let content = content.trim_start();
    if !content.starts_with("---") {
        return None;
    }

    let rest = content[3..].trim_start();
    if let Some(end) = rest.find("---") {
        let yaml_content = &rest[..end];
        serde_yaml::from_str(yaml_content).ok()
    } else {
        None
    }
}

fn check_requirements(metadata: &SkillMetadata) -> (bool, SkillRequirements) {
    let mut missing = SkillRequirements::default();
    let mut all_satisfied = true;

    for bin in &metadata.requires.bins {
        if which::which(bin).is_err() {
            missing.bins.push(bin.clone());
            all_satisfied = false;
        }
    }

    for env in &metadata.requires.env {
        if std::env::var(env).is_err() {
            missing.env.push(env.clone());
            all_satisfied = false;
        }
    }

    #[cfg(target_os = "windows")]
    let current_os = "windows";
    #[cfg(target_os = "macos")]
    let current_os = "macos";
    #[cfg(target_os = "linux")]
    let current_os = "linux";

    if !metadata.requires.os.is_empty() && !metadata.requires.os.contains(&current_os.to_string()) {
        missing.os.push(current_os.to_string());
        all_satisfied = false;
    }

    (all_satisfied, missing)
}

fn load_skill_from_file(path: &Path) -> Option<SkillEntry> {
    let content = std::fs::read_to_string(path).ok()?;
    let metadata = parse_frontmatter(&content)?;

    if metadata.name.is_empty() {
        return None;
    }

    let (eligible, missing) = check_requirements(&metadata);

    Some(SkillEntry {
        name: metadata.name.clone(),
        description: metadata.description.clone(),
        path: path.to_path_buf(),
        metadata,
        eligible,
        missing,
    })
}

pub fn load_skills_from_dir(dir: &Path) -> Vec<SkillEntry> {
    let mut skills = Vec::new();

    if !dir.exists() {
        return skills;
    }

    for entry in WalkDir::new(dir)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.file_name().and_then(|s| s.to_str()) == Some("SKILL.md") {
            if let Some(skill) = load_skill_from_file(path) {
                skills.push(skill);
            }
        }
    }

    skills.sort_by(|a, b| a.name.cmp(&b.name));
    skills
}

pub fn build_skills_report() -> SkillsReport {
    use crate::utils::CONFIG_DIR;

    let bundled_dir = PathBuf::from(CONFIG_DIR.as_str()).join("skills");
    let project_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_default()
        .join("skills");

    let mut all_skills = Vec::new();

    all_skills.extend(load_skills_from_dir(&project_dir));
    all_skills.extend(load_skills_from_dir(&bundled_dir));

    SkillsReport {
        skills_dir: project_dir.to_string_lossy().to_string(),
        bundled_dir: bundled_dir.to_string_lossy().to_string(),
        skills: all_skills,
    }
}

pub fn format_skills_list(report: &SkillsReport, eligible_only: bool) -> String {
    let skills: Vec<_> = if eligible_only {
        report.skills.iter().filter(|s| s.eligible).collect()
    } else {
        report.skills.iter().collect()
    };

    if skills.is_empty() {
        return if eligible_only {
            "No eligible skills found.".to_string()
        } else {
            "No skills found.".to_string()
        };
    }

    let eligible_count = report.skills.iter().filter(|s| s.eligible).count();

    let mut output = format!("Skills ({}/{})\n", eligible_count, report.skills.len());
    output.push_str(&"=".repeat(50));
    output.push('\n');
    output.push('\n');

    for skill in skills {
        let status = if skill.eligible {
            "✓ ready"
        } else if !skill.missing.bins.is_empty() {
            "✗ missing bins"
        } else if !skill.missing.env.is_empty() {
            "✗ missing env"
        } else {
            "✗ missing"
        };

        let emoji = skill.metadata.emoji.as_deref().unwrap_or("📦");
        output.push_str(&format!("{} {} - {}\n", emoji, skill.name, status));

        if !skill.description.is_empty() {
            output.push_str(&format!(
                "   {}\n",
                &skill.description[..skill.description.len().min(60)]
            ));
        }

        if !skill.missing.bins.is_empty() {
            output.push_str(&format!(
                "   Missing bins: {}\n",
                skill.missing.bins.join(", ")
            ));
        }
        if !skill.missing.env.is_empty() {
            output.push_str(&format!(
                "   Missing env: {}\n",
                skill.missing.env.join(", ")
            ));
        }
        output.push('\n');
    }

    output.push_str("Tip: use clawdhub to search, install, and sync skills.\n");

    output
}

pub fn format_skill_info(report: &SkillsReport, name: &str) -> String {
    let skill = report.skills.iter().find(|s| s.name == name);

    match skill {
        Some(s) => {
            let mut output = String::new();
            let emoji = s.metadata.emoji.as_deref().unwrap_or("📦");
            let status = if s.eligible {
                "✓ Ready"
            } else {
                "✗ Missing requirements"
            };

            output.push_str(&format!("{} {} {}\n\n", emoji, s.name, status));
            output.push_str(&format!("{}\n\n", s.description));

            output.push_str("Details:\n");
            output.push_str(&format!("  Source: {}\n", s.path.display()));
            if let Some(homepage) = &s.metadata.homepage {
                output.push_str(&format!("  Homepage: {}\n", homepage));
            }

            if !s.metadata.requires.bins.is_empty() {
                output.push_str("\nRequirements - Binaries:\n");
                for bin in &s.metadata.requires.bins {
                    let installed = which::which(bin).is_ok();
                    let mark = if installed { "✓" } else { "✗" };
                    output.push_str(&format!("  {} {}\n", mark, bin));
                }
            }

            if !s.metadata.requires.env.is_empty() {
                output.push_str("\nRequirements - Environment:\n");
                for env in &s.metadata.requires.env {
                    let installed = std::env::var(env).is_ok();
                    let mark = if installed { "✓" } else { "✗" };
                    output.push_str(&format!("  {} ${}\n", mark, env));
                }
            }

            output
        }
        None => format!("Skill \"{}\" not found.", name),
    }
}
