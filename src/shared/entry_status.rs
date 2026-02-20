//! Port of `openclaw/src/shared/entry-status.ts`
//!
//! Evaluates entry metadata requirements for plugins/extensions, combining
//! emoji/homepage resolution with requirement checks.

// ─── Types ──────────────────────────────────────────────────────────────────

/// Requirements metadata from a plugin/extension manifest.
#[derive(Debug, Clone, Default)]
pub struct RequirementsMetadata {
    pub bins: Vec<String>,
    pub any_bins: Vec<String>,
    pub env: Vec<String>,
    pub config: Vec<String>,
    pub os: Vec<String>,
    pub emoji: Option<String>,
    pub homepage: Option<String>,
}

/// Resolved set of requirement names.
#[derive(Debug, Clone, Default)]
pub struct Requirements {
    pub bins: Vec<String>,
    pub any_bins: Vec<String>,
    pub env: Vec<String>,
    pub config: Vec<String>,
}

/// A remote node's capabilities for requirement evaluation.
#[derive(Debug, Clone, Default)]
pub struct RequirementRemote {
    pub bins: Vec<String>,
}

/// Individual config-path check result.
#[derive(Debug, Clone)]
pub struct RequirementConfigCheck {
    pub path: String,
    pub satisfied: bool,
}

/// Result from requirement evaluation.
#[derive(Debug)]
pub struct RequirementEvalResult {
    pub required: Requirements,
    pub missing: Requirements,
    pub eligible: bool,
    pub config_checks: Vec<RequirementConfigCheck>,
}

// ─── Emoji / homepage resolution ────────────────────────────────────────────

/// Resolve emoji and homepage from metadata and/or frontmatter fields.
///
/// Priority: metadata fields first, then frontmatter emoji, then
/// frontmatter homepage / website / url.
pub fn resolve_emoji_and_homepage(
    metadata_emoji: Option<&str>,
    metadata_homepage: Option<&str>,
    frontmatter_emoji: Option<&str>,
    frontmatter_homepage: Option<&str>,
    frontmatter_website: Option<&str>,
    frontmatter_url: Option<&str>,
) -> (Option<String>, Option<String>) {
    let emoji = metadata_emoji
        .or(frontmatter_emoji)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());

    let homepage = metadata_homepage
        .or(frontmatter_homepage)
        .or(frontmatter_website)
        .or(frontmatter_url)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());

    (emoji, homepage)
}

// ─── Requirement evaluation ─────────────────────────────────────────────────

/// Evaluate requirements from metadata, including remote node support.
pub fn evaluate_requirements_from_metadata_with_remote(
    always: bool,
    metadata: Option<&RequirementsMetadata>,
    has_local_bin: &dyn Fn(&str) -> bool,
    local_platform: &str,
    remote: Option<&RequirementRemote>,
    is_env_satisfied: &dyn Fn(&str) -> bool,
    is_config_satisfied: &dyn Fn(&str) -> bool,
) -> RequirementEvalResult {
    let metadata = match metadata {
        Some(m) => m,
        None => {
            return RequirementEvalResult {
                required: Requirements::default(),
                missing: Requirements::default(),
                eligible: true, // no requirements = always eligible
                config_checks: Vec::new(),
            };
        }
    };

    // OS filter
    if !metadata.os.is_empty() {
        let platform_matches = metadata
            .os
            .iter()
            .any(|os| os.eq_ignore_ascii_case(local_platform));
        if !platform_matches && !always {
            return RequirementEvalResult {
                required: Requirements {
                    bins: metadata.bins.clone(),
                    any_bins: metadata.any_bins.clone(),
                    env: metadata.env.clone(),
                    config: metadata.config.clone(),
                },
                missing: Requirements {
                    bins: metadata.bins.clone(),
                    any_bins: metadata.any_bins.clone(),
                    env: metadata.env.clone(),
                    config: metadata.config.clone(),
                },
                eligible: false,
                config_checks: Vec::new(),
            };
        }
    }

    let remote_bins: Vec<&str> = remote
        .map(|r| r.bins.iter().map(|s| s.as_str()).collect())
        .unwrap_or_default();

    // Check bins
    let mut missing_bins = Vec::new();
    for bin in &metadata.bins {
        let local_ok = has_local_bin(bin);
        let remote_ok = remote_bins.contains(&bin.as_str());
        if !local_ok && !remote_ok {
            missing_bins.push(bin.clone());
        }
    }

    // Check any_bins
    let mut missing_any_bins = Vec::new();
    if !metadata.any_bins.is_empty() {
        let any_found = metadata.any_bins.iter().any(|b| {
            has_local_bin(b) || remote_bins.contains(&b.as_str())
        });
        if !any_found {
            missing_any_bins = metadata.any_bins.clone();
        }
    }

    // Check env
    let mut missing_env = Vec::new();
    for env_name in &metadata.env {
        if !is_env_satisfied(env_name) {
            missing_env.push(env_name.clone());
        }
    }

    // Check config
    let mut missing_config = Vec::new();
    let mut config_checks = Vec::new();
    for config_path in &metadata.config {
        let satisfied = is_config_satisfied(config_path);
        config_checks.push(RequirementConfigCheck {
            path: config_path.clone(),
            satisfied,
        });
        if !satisfied {
            missing_config.push(config_path.clone());
        }
    }

    let eligible = missing_bins.is_empty()
        && missing_any_bins.is_empty()
        && missing_env.is_empty()
        && missing_config.is_empty();

    RequirementEvalResult {
        required: Requirements {
            bins: metadata.bins.clone(),
            any_bins: metadata.any_bins.clone(),
            env: metadata.env.clone(),
            config: metadata.config.clone(),
        },
        missing: Requirements {
            bins: missing_bins,
            any_bins: missing_any_bins,
            env: missing_env,
            config: missing_config,
        },
        eligible,
        config_checks,
    }
}

// ─── Evaluate entry metadata ────────────────────────────────────────────────

/// Parameters for [`evaluate_entry_metadata_requirements`].
pub struct EntryMetadataRequirementsParams<'a> {
    pub always: bool,
    pub metadata: Option<&'a RequirementsMetadata>,
    pub metadata_emoji: Option<&'a str>,
    pub metadata_homepage: Option<&'a str>,
    pub frontmatter_emoji: Option<&'a str>,
    pub frontmatter_homepage: Option<&'a str>,
    pub frontmatter_website: Option<&'a str>,
    pub frontmatter_url: Option<&'a str>,
    pub has_local_bin: &'a dyn Fn(&str) -> bool,
    pub local_platform: &'a str,
    pub remote: Option<&'a RequirementRemote>,
    pub is_env_satisfied: &'a dyn Fn(&str) -> bool,
    pub is_config_satisfied: &'a dyn Fn(&str) -> bool,
}

/// Result of evaluating entry metadata requirements.
#[derive(Debug)]
pub struct EntryMetadataRequirementsResult {
    pub emoji: Option<String>,
    pub homepage: Option<String>,
    pub required: Requirements,
    pub missing: Requirements,
    pub requirements_satisfied: bool,
    pub config_checks: Vec<RequirementConfigCheck>,
}

/// Evaluate entry metadata requirements — resolve emoji/homepage and check
/// whether all runtime requirements are satisfied.
pub fn evaluate_entry_metadata_requirements(
    params: &EntryMetadataRequirementsParams,
) -> EntryMetadataRequirementsResult {
    let (emoji, homepage) = resolve_emoji_and_homepage(
        params.metadata_emoji,
        params.metadata_homepage,
        params.frontmatter_emoji,
        params.frontmatter_homepage,
        params.frontmatter_website,
        params.frontmatter_url,
    );

    let eval_result = evaluate_requirements_from_metadata_with_remote(
        params.always,
        params.metadata,
        params.has_local_bin,
        params.local_platform,
        params.remote,
        params.is_env_satisfied,
        params.is_config_satisfied,
    );

    EntryMetadataRequirementsResult {
        emoji,
        homepage,
        required: eval_result.required,
        missing: eval_result.missing,
        requirements_satisfied: eval_result.eligible,
        config_checks: eval_result.config_checks,
    }
}

/// Convenience wrapper that uses the current platform automatically.
pub fn evaluate_entry_metadata_requirements_for_current_platform(
    params: &EntryMetadataRequirementsParams,
) -> EntryMetadataRequirementsResult {
    let current_platform = if cfg!(target_os = "windows") {
        "win32"
    } else if cfg!(target_os = "macos") {
        "darwin"
    } else {
        "linux"
    };

    let adjusted = EntryMetadataRequirementsParams {
        always: params.always,
        metadata: params.metadata,
        metadata_emoji: params.metadata_emoji,
        metadata_homepage: params.metadata_homepage,
        frontmatter_emoji: params.frontmatter_emoji,
        frontmatter_homepage: params.frontmatter_homepage,
        frontmatter_website: params.frontmatter_website,
        frontmatter_url: params.frontmatter_url,
        has_local_bin: params.has_local_bin,
        local_platform: current_platform,
        remote: params.remote,
        is_env_satisfied: params.is_env_satisfied,
        is_config_satisfied: params.is_config_satisfied,
    };

    evaluate_entry_metadata_requirements(&adjusted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emoji_from_metadata() {
        let (emoji, _) = resolve_emoji_and_homepage(
            Some("🦀"),
            None,
            Some("🐍"), // should be ignored
            None,
            None,
            None,
        );
        assert_eq!(emoji.as_deref(), Some("🦀"));
    }

    #[test]
    fn emoji_falls_back_to_frontmatter() {
        let (emoji, _) = resolve_emoji_and_homepage(
            None, None,
            Some("🐍"),
            None, None, None,
        );
        assert_eq!(emoji.as_deref(), Some("🐍"));
    }

    #[test]
    fn homepage_from_website_fallback() {
        let (_, homepage) = resolve_emoji_and_homepage(
            None, None, None, None,
            Some("https://example.com"),
            None,
        );
        assert_eq!(homepage.as_deref(), Some("https://example.com"));
    }

    #[test]
    fn no_metadata_always_eligible() {
        let params = EntryMetadataRequirementsParams {
            always: false,
            metadata: None,
            metadata_emoji: None,
            metadata_homepage: None,
            frontmatter_emoji: Some("🦀"),
            frontmatter_homepage: None,
            frontmatter_website: None,
            frontmatter_url: None,
            has_local_bin: &|_| true,
            local_platform: "linux",
            remote: None,
            is_env_satisfied: &|_| true,
            is_config_satisfied: &|_| true,
        };
        let result = evaluate_entry_metadata_requirements(&params);
        assert!(result.requirements_satisfied);
        assert_eq!(result.emoji.as_deref(), Some("🦀"));
    }

    #[test]
    fn missing_bin_not_eligible() {
        let meta = RequirementsMetadata {
            bins: vec!["some-tool".to_string()],
            ..Default::default()
        };
        let params = EntryMetadataRequirementsParams {
            always: false,
            metadata: Some(&meta),
            metadata_emoji: None,
            metadata_homepage: None,
            frontmatter_emoji: None,
            frontmatter_homepage: None,
            frontmatter_website: None,
            frontmatter_url: None,
            has_local_bin: &|_| false,
            local_platform: "linux",
            remote: None,
            is_env_satisfied: &|_| true,
            is_config_satisfied: &|_| true,
        };
        let result = evaluate_entry_metadata_requirements(&params);
        assert!(!result.requirements_satisfied);
        assert_eq!(result.missing.bins, vec!["some-tool"]);
    }

    #[test]
    fn os_filter_rejects_wrong_platform() {
        let meta = RequirementsMetadata {
            os: vec!["darwin".to_string()],
            ..Default::default()
        };
        let params = EntryMetadataRequirementsParams {
            always: false,
            metadata: Some(&meta),
            metadata_emoji: None,
            metadata_homepage: None,
            frontmatter_emoji: None,
            frontmatter_homepage: None,
            frontmatter_website: None,
            frontmatter_url: None,
            has_local_bin: &|_| true,
            local_platform: "linux",
            remote: None,
            is_env_satisfied: &|_| true,
            is_config_satisfied: &|_| true,
        };
        let result = evaluate_entry_metadata_requirements(&params);
        assert!(!result.requirements_satisfied);
    }

    #[test]
    fn remote_bin_satisfies() {
        let meta = RequirementsMetadata {
            bins: vec!["ffmpeg".to_string()],
            ..Default::default()
        };
        let remote = RequirementRemote {
            bins: vec!["ffmpeg".to_string()],
        };
        let params = EntryMetadataRequirementsParams {
            always: false,
            metadata: Some(&meta),
            metadata_emoji: None,
            metadata_homepage: None,
            frontmatter_emoji: None,
            frontmatter_homepage: None,
            frontmatter_website: None,
            frontmatter_url: None,
            has_local_bin: &|_| false,
            local_platform: "linux",
            remote: Some(&remote),
            is_env_satisfied: &|_| true,
            is_config_satisfied: &|_| true,
        };
        let result = evaluate_entry_metadata_requirements(&params);
        assert!(result.requirements_satisfied);
    }
}
