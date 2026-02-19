# Hello Plugin Example

This is a simple example plugin demonstrating the krabkrab plugin system.

## Structure

```
hello-plugin/
├── plugin.json    # Plugin manifest
└── lib.rs         # Plugin implementation (for static plugins)
```

## Manifest (plugin.json)

```json
{
  "name": "hello-plugin",
  "version": "1.0.0",
  "description": "A simple example plugin that provides a greeting tool",
  "author": "krabkrab Team",
  "enabled": true,
  "kind": "extension",
  "requires": ["tools"],
  "entry": null
}
```

## Features

This plugin provides two tools:

### 1. `greet`

Generates a greeting message in multiple languages.

**Parameters:**
- `name` (required): Name of the person to greet
- `language` (optional): Language code (en, es, fr, de)

**Example:**
```json
{
  "name": "Alice",
  "language": "es"
}
```

**Response:**
```json
{
  "greeting": "¡Hola, Alice!",
  "language": "es",
  "name": "Alice"
}
```

### 2. `farewell`

Generates a farewell message.

**Parameters:**
- `name` (required): Name of the person to bid farewell

## Plugin Kinds

- `extension`: General-purpose plugin
- `connector`: Messaging platform connector
- `provider`: LLM provider
- `tool`: Tool provider
- `auth`: Authentication provider

## Loading the Plugin

### Static Linking (Built-in)

For static plugins, add the plugin module to your codebase and register it:

```rust
use krabkrab::plugins::{PluginManager, PluginRegistry};

// Register the plugin
manager.registry_mut().register(hello_plugin::manifest()).unwrap();
```

### Dynamic Loading

For dynamic plugins, place the plugin in one of the plugin directories:

- `./plugins/`
- `~/.krabkrab/plugins/`

Then load it:

```rust
use krabkrab::plugins::loader::PluginManager;

let mut manager = PluginManager::new();
let summary = manager.load_all()?;
println!("Loaded {} plugins", summary.loaded);
```

## Native ABI (dynamic libraries)

Native plugins should export these symbols:

- `krabkrab_plugin_manifest_json` -> returns `*const c_char` with `PluginManifest` JSON
- `krabkrab_plugin_declaration_json` -> returns `*const c_char` with `PluginDeclaration` JSON

Example signature:

```rust
#[no_mangle]
pub extern "C" fn krabkrab_plugin_declaration_json() -> *const std::os::raw::c_char {
    // return pointer to static, null-terminated JSON string
}
```

## Testing

Run the plugin tests:

```bash
cargo test -p krabkrab plugins::
```
