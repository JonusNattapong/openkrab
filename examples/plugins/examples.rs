//! Plugin system examples and usage demonstrations.

use krabkrab::plugins::loader::{PluginLoader, PluginLoaderConfig, PluginManager};
use krabkrab::plugins::{PluginKind, PluginManifest, PluginRegistry};
use std::path::PathBuf;

/// Example: Basic plugin discovery and loading.
pub fn example_basic_loading() {
    println!("=== Basic Plugin Loading Example ===\n");

    // Create a plugin loader with default configuration
    let loader = PluginLoader::new();

    println!("Plugin directories:");
    for dir in &loader.config().plugin_dirs {
        println!("  - {}", dir.display());
    }

    // Discover plugins
    match loader.discover() {
        Ok(plugins) => {
            println!("\nDiscovered {} plugins:", plugins.len());
            for plugin in plugins {
                println!(
                    "  - {} v{} ({})",
                    plugin.manifest.name, plugin.manifest.version, plugin.manifest.description
                );
            }
        }
        Err(e) => {
            eprintln!("Failed to discover plugins: {}", e);
        }
    }
}

/// Example: Load plugins with custom directories.
pub fn example_custom_directories() {
    println!("\n=== Custom Directories Example ===\n");

    let config = PluginLoaderConfig {
        plugin_dirs: vec![
            PathBuf::from("./my-plugins"),
            PathBuf::from("/usr/share/krabkrab/plugins"),
        ],
        hot_reload: false,
        extensions: vec!["wasm".to_string(), "so".to_string()],
    };

    let mut loader = PluginLoader::with_config(config);
    let mut registry = PluginRegistry::new();

    // Load all discovered plugins
    match loader.load_all(&mut registry) {
        Ok(summary) => {
            println!("Loaded: {}", summary.loaded);
            println!("Failed: {}", summary.failed);
            for (name, error) in &summary.errors {
                println!("  - {}: {}", name, error);
            }
        }
        Err(e) => {
            eprintln!("Failed to load plugins: {}", e);
        }
    }
}

/// Example: Using the PluginManager.
pub fn example_plugin_manager() {
    println!("\n=== Plugin Manager Example ===\n");

    let mut manager = PluginManager::new();

    // Load all plugins from configured directories
    match manager.load_all() {
        Ok(summary) => {
            println!("Plugin loading summary:");
            println!("  Loaded: {}", summary.loaded);
            println!("  Failed: {}", summary.failed);

            // List all registered plugins
            println!("\nRegistered plugins:");
            for name in manager.registry().list() {
                let entry = manager.registry().get(name).unwrap();
                let status = if entry.is_enabled() {
                    "enabled"
                } else {
                    "disabled"
                };
                println!("  - {} ({})", name, status);
            }
        }
        Err(e) => {
            eprintln!("Failed to load plugins: {}", e);
        }
    }
}

/// Example: Programmatically creating and registering a plugin.
pub fn example_programmatic_plugin() {
    println!("\n=== Programmatic Plugin Example ===\n");

    let mut registry = PluginRegistry::new();

    // Create a plugin manifest programmatically
    let manifest = PluginManifest {
        name: "my-custom-plugin".to_string(),
        version: "1.0.0".to_string(),
        description: "A dynamically created plugin".to_string(),
        author: Some("Developer".to_string()),
        enabled: true,
        kind: PluginKind::Tool,
        requires: vec![],
        entry: None,
    };

    // Register the plugin
    match registry.register(manifest) {
        Ok(()) => {
            println!("Successfully registered plugin");
            println!("Total plugins: {}", registry.len());
        }
        Err(e) => {
            eprintln!("Failed to register plugin: {}", e);
        }
    }
}

/// Run all examples.
pub fn run_all_examples() {
    example_basic_loading();
    example_custom_directories();
    example_plugin_manager();
    example_programmatic_plugin();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_plugin_loading_example() {
        // Create a temporary plugin directory
        let temp_dir = TempDir::new().unwrap();
        let plugin_dir = temp_dir.path().join("test-plugin");
        std::fs::create_dir(&plugin_dir).unwrap();

        // Create a plugin manifest
        let manifest = r#"{
            "name": "example-test-plugin",
            "version": "1.0.0",
            "description": "Test plugin for examples"
        }"#;

        let mut file = std::fs::File::create(plugin_dir.join("plugin.json")).unwrap();
        file.write_all(manifest.as_bytes()).unwrap();

        // Create loader with temp directory
        let config = PluginLoaderConfig {
            plugin_dirs: vec![temp_dir.path().to_path_buf()],
            hot_reload: false,
            extensions: vec![],
        };

        let loader = PluginLoader::with_config(config);
        let discovered = loader.discover().unwrap();

        assert_eq!(discovered.len(), 1);
        assert_eq!(discovered[0].manifest.name, "example-test-plugin");
    }
}
