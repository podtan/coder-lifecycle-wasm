# coder-lifecycle-wasm

A WASM extension for ABK (Agent Builder Kit) that provides lifecycle management capabilities for coding agents.

## Features

- **Template Loading**: Load embedded markdown templates for different task types
- **Template Rendering**: Variable substitution in templates using `{variable}` syntax
- **Section Extraction**: Extract sections from markdown templates
- **Task Classification**: Classify tasks based on description keywords
- **System Info**: Get basic system information for template variables

## Architecture

This extension implements the ABK Extension System interfaces:

- **Core Interface** (required): Provides metadata, capabilities list, and initialization
- **Lifecycle Interface**: Provides template management and task classification

## Building

```bash
# Build WASM
./build.sh

# Output will be in wasm-output/
ls -la wasm-output/
```

## Installation

Copy the following files to your ABK extensions directory:

```bash
cp wasm-output/coder_lifecycle_wasm.wasm ~/.abk/extensions/coder-lifecycle/
cp extension.toml ~/.abk/extensions/coder-lifecycle/
```

Or use the ABK extension manager:

```bash
abk extension install coder-lifecycle ./wasm-output/
```

## Usage

Once installed, the extension will be automatically discovered by ABK:

```rust
use abk::extension::ExtensionManager;

// Create extension manager
let mut manager = ExtensionManager::new("~/.abk/extensions").await?;

// Discover extensions
let manifests = manager.discover().await?;

// Instantiate the lifecycle extension
let instance = manager.instantiate("coder-lifecycle")?;

// Load a template
let template = instance.load_template("system")?;

// Render with variables
let rendered = instance.render_template(
    &template,
    &[
        ("project_name".to_string(), "MyProject".to_string()),
        ("language".to_string(), "Rust".to_string()),
    ],
)?;

// Classify a task
let (task_type, confidence) = instance.classify_task("Fix the login bug")?;
assert_eq!(task_type, "bug_fix");
```

## Supported Templates

| Template Name | Description |
|---------------|-------------|
| `system` | Main system prompt template |
| `system_classification` | Task classification prompt |
| `action_observation` | Action/observation format |
| `format_error` | Error formatting template |
| `useful_commands` | Available commands reference |
| `task/bug_fix` | Bug fix task template |
| `task/feature` | Feature implementation template |
| `task/maintenance` | Maintenance task template |
| `task/query` | Query/question template |
| `task/fallback` | Default fallback template |

## Task Types

The extension classifies tasks into the following types based on keywords:

| Type | Keywords | Confidence |
|------|----------|------------|
| `bug_fix` | fix, bug, error, issue | 0.8 |
| `feature` | add, implement, create, feature | 0.8 |
| `maintenance` | refactor, cleanup, update, maintain | 0.7 |
| `query` | how, what, why, explain, show | 0.7 |
| `fallback` | (none matched) | 0.5 |

## Extension Manifest

```toml
[extension]
id = "coder-lifecycle"
name = "Coder Lifecycle Extension"
version = "0.3.0"
api_version = "0.3.0"

[lib]
kind = "rust"
path = "coder_lifecycle_wasm.wasm"

[capabilities]
lifecycle = true
```

## WIT Interfaces

This extension implements the following WIT interfaces from ABK:

- `abk:extension/core@0.3.0` - Required core interface
- `abk:extension/lifecycle@0.3.0` - Lifecycle capability interface

## Development

```bash
# Install WASM target
rustup target add wasm32-wasip1

# Build in debug mode
cargo build

# Run tests (native)
cargo test

# Build release WASM
./build.sh
```

## License

MIT OR Apache-2.0
