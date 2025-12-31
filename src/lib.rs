//! Coder Lifecycle Extension for ABK
//!
//! This extension provides lifecycle management capabilities for coding agents,
//! including template loading, rendering, and task classification.

wit_bindgen::generate!({
    world: "lifecycle-extension",
    path: "wit",
});

use exports::abk::extension::core::{ExtensionMetadata, Guest as CoreGuest};
use exports::abk::extension::lifecycle::{
    Guest as LifecycleGuest, LifecycleError, TaskClassification, TemplateSection, TemplateVariable,
};

use regex::Regex;
use std::collections::HashMap;

/// The lifecycle extension implementation
struct CoderLifecycleExtension;

// Embed templates at compile time
const SYSTEM_TEMPLATE: &str = include_str!("../templates/system.md");
const SYSTEM_CLASSIFICATION_TEMPLATE: &str = include_str!("../templates/system_classification.md");
const ACTION_OBSERVATION_TEMPLATE: &str = include_str!("../templates/action_observation.md");
const FORMAT_ERROR_TEMPLATE: &str = include_str!("../templates/format_error.md");
const USEFUL_COMMANDS_TEMPLATE: &str = include_str!("../templates/useful_commands.md");

const TASK_BUG_FIX_TEMPLATE: &str = include_str!("../templates/task/bug_fix.md");
const TASK_FEATURE_TEMPLATE: &str = include_str!("../templates/task/feature.md");
const TASK_MAINTENANCE_TEMPLATE: &str = include_str!("../templates/task/maintenance.md");
const TASK_QUERY_TEMPLATE: &str = include_str!("../templates/task/query.md");
const TASK_FALLBACK_TEMPLATE: &str = include_str!("../templates/task/fallback.md");

// ===== Core Interface Implementation =====

impl CoreGuest for CoderLifecycleExtension {
    fn get_metadata() -> ExtensionMetadata {
        ExtensionMetadata {
            id: "coder-lifecycle".to_string(),
            name: "Coder Lifecycle Extension".to_string(),
            version: "0.3.0".to_string(),
            api_version: "0.3.0".to_string(),
            description: "Template lifecycle management for ABK coding agents".to_string(),
        }
    }

    fn list_capabilities() -> Vec<String> {
        vec!["lifecycle".to_string()]
    }

    fn init() -> Result<(), String> {
        // No initialization needed for this extension
        Ok(())
    }
}

// ===== Lifecycle Interface Implementation =====

impl LifecycleGuest for CoderLifecycleExtension {
    fn load_template(template_name: String) -> Result<String, LifecycleError> {
        let content = match template_name.as_str() {
            "system" => SYSTEM_TEMPLATE,
            "system_classification" => SYSTEM_CLASSIFICATION_TEMPLATE,
            "action_observation" => ACTION_OBSERVATION_TEMPLATE,
            "format_error" => FORMAT_ERROR_TEMPLATE,
            "useful_commands" => USEFUL_COMMANDS_TEMPLATE,
            "task/bug_fix" => TASK_BUG_FIX_TEMPLATE,
            "task/feature" => TASK_FEATURE_TEMPLATE,
            "task/maintenance" => TASK_MAINTENANCE_TEMPLATE,
            "task/query" => TASK_QUERY_TEMPLATE,
            "task/fallback" => TASK_FALLBACK_TEMPLATE,
            _ => {
                return Err(LifecycleError {
                    message: format!("Template not found: {}", template_name),
                    code: Some("TEMPLATE_NOT_FOUND".to_string()),
                })
            }
        };

        Ok(content.to_string())
    }

    fn render_template(content: String, variables: Vec<TemplateVariable>) -> String {
        let mut result = content;

        // Convert variables to HashMap for easier lookup
        let var_map: HashMap<String, String> =
            variables.into_iter().map(|v| (v.key, v.value)).collect();

        // Simple variable substitution using {variable} syntax
        for (key, value) in var_map {
            let placeholder = format!("{{{}}}", key);
            result = result.replace(&placeholder, &value);
        }

        result
    }

    fn extract_sections(content: String) -> Vec<TemplateSection> {
        let mut sections = Vec::new();
        let mut current_section: Option<String> = None;
        let mut current_content = Vec::new();

        for line in content.lines() {
            if line.starts_with("## ") {
                // Save previous section
                if let Some(section_name) = &current_section {
                    sections.push(TemplateSection {
                        name: section_name.clone(),
                        content: current_content.join("\n").trim().to_string(),
                    });
                }

                // Start new section
                current_section = Some(line[3..].trim().to_string());
                current_content.clear();
            } else if current_section.is_some() {
                current_content.push(line);
            }
        }

        // Save last section
        if let Some(section_name) = &current_section {
            sections.push(TemplateSection {
                name: section_name.to_string(),
                content: current_content.join("\n").trim().to_string(),
            });
        }

        sections
    }

    fn validate_template_variables(content: String) -> Vec<String> {
        let re = Regex::new(r"\{(\w+)\}").unwrap();
        let mut variables = Vec::new();

        for caps in re.captures_iter(&content) {
            if let Some(var_name) = caps.get(1) {
                let var = var_name.as_str().to_string();
                if !variables.contains(&var) {
                    variables.push(var);
                }
            }
        }

        variables
    }

    fn get_task_template_name(task_type: String) -> String {
        match task_type.as_str() {
            "bug_fix" => "task/bug_fix".to_string(),
            "feature" => "task/feature".to_string(),
            "maintenance" => "task/maintenance".to_string(),
            "query" => "task/query".to_string(),
            _ => "task/fallback".to_string(),
        }
    }

    fn classify_task(task_description: String) -> Result<TaskClassification, LifecycleError> {
        // Simple keyword-based classification
        // In production, this could use ML or more sophisticated heuristics
        let desc_lower = task_description.to_lowercase();

        let (task_type, confidence) = if desc_lower.contains("fix")
            || desc_lower.contains("bug")
            || desc_lower.contains("error")
            || desc_lower.contains("issue")
        {
            ("bug_fix", 0.8)
        } else if desc_lower.contains("add")
            || desc_lower.contains("implement")
            || desc_lower.contains("create")
            || desc_lower.contains("feature")
        {
            ("feature", 0.8)
        } else if desc_lower.contains("refactor")
            || desc_lower.contains("cleanup")
            || desc_lower.contains("update")
            || desc_lower.contains("maintain")
        {
            ("maintenance", 0.7)
        } else if desc_lower.contains("how")
            || desc_lower.contains("what")
            || desc_lower.contains("why")
            || desc_lower.contains("explain")
            || desc_lower.contains("show")
        {
            ("query", 0.7)
        } else {
            ("fallback", 0.5)
        };

        Ok(TaskClassification {
            task_type: task_type.to_string(),
            confidence,
        })
    }

    fn load_useful_commands() -> Result<String, LifecycleError> {
        Ok(USEFUL_COMMANDS_TEMPLATE.to_string())
    }

    fn get_system_info_variables() -> Vec<TemplateVariable> {
        let mut vars = Vec::new();

        // In WASM, we can only get compile-time target info
        // Runtime system info must be provided by the host
        #[cfg(target_os = "linux")]
        {
            vars.push(TemplateVariable {
                key: "system".to_string(),
                value: "Linux".to_string(),
            });
        }
        #[cfg(target_os = "macos")]
        {
            vars.push(TemplateVariable {
                key: "system".to_string(),
                value: "Darwin".to_string(),
            });
        }
        #[cfg(target_os = "windows")]
        {
            vars.push(TemplateVariable {
                key: "system".to_string(),
                value: "Windows".to_string(),
            });
        }
        // WASM target
        #[cfg(target_arch = "wasm32")]
        {
            vars.push(TemplateVariable {
                key: "system".to_string(),
                value: "wasm32".to_string(),
            });
        }

        #[cfg(target_arch = "x86_64")]
        {
            vars.push(TemplateVariable {
                key: "machine".to_string(),
                value: "x86_64".to_string(),
            });
        }
        #[cfg(target_arch = "aarch64")]
        {
            vars.push(TemplateVariable {
                key: "machine".to_string(),
                value: "aarch64".to_string(),
            });
        }
        #[cfg(target_arch = "wasm32")]
        {
            vars.push(TemplateVariable {
                key: "machine".to_string(),
                value: "wasm32".to_string(),
            });
        }

        // Add placeholder values for fields we can't determine in WASM
        vars.push(TemplateVariable {
            key: "release".to_string(),
            value: "unknown".to_string(),
        });
        vars.push(TemplateVariable {
            key: "version".to_string(),
            value: "unknown".to_string(),
        });
        vars.push(TemplateVariable {
            key: "processor".to_string(),
            value: "unknown".to_string(),
        });

        vars
    }
}

export!(CoderLifecycleExtension);
