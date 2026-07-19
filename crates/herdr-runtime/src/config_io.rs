use std::path::Path;

use herdr_state::AppState;

pub fn update_config_file(
    state: &mut AppState,
    error_context: &str,
    config_path: impl Fn() -> String,
    read_config: impl Fn(&str) -> Option<String>,
    write_config: impl Fn(&str, &str) -> Result<(), String>,
    log_error: impl Fn(&Path, &str, &str),
    set_error_deadline: impl FnOnce(std::time::Instant),
    update: impl FnOnce(&str) -> String,
) -> bool {
    let path_str = config_path();
    let path = Path::new(&path_str);
    if let Some(parent) = path.parent() {
        if let Err(err) = std::fs::create_dir_all(parent) {
            log_error(path, error_context, &err.to_string());
            state.config_diagnostic = Some(format!("failed to save {error_context}: {err}"));
            set_error_deadline(std::time::Instant::now() + std::time::Duration::from_secs(5));
            return false;
        }
    }

    let content = read_config(&path_str).unwrap_or_default();
    let new_content = update(&content);
    if let Err(err) = write_config(&path_str, &new_content) {
        log_error(path, error_context, &err);
        state.config_diagnostic = Some(format!("failed to save {error_context}: {err}"));
        set_error_deadline(std::time::Instant::now() + std::time::Duration::from_secs(5));
        return false;
    }

    true
}

pub fn config_update_theme(
    content: &str,
    name: &str,
) -> String {
    let content = upsert_section_value(content, "theme", "name", &format!("\"{name}\""));
    upsert_section_bool(&content, "theme", "auto_switch", false)
}

pub fn config_update_sound(content: &str, enabled: bool) -> String {
    upsert_section_bool(content, "ui.sound", "enabled", enabled)
}

pub fn config_update_toast_delivery(content: &str, delivery: herdr_state::ToastDelivery) -> String {
    let value = match delivery {
        herdr_state::ToastDelivery::Off => "\"off\"",
        herdr_state::ToastDelivery::Herdr => "\"herdr\"",
        herdr_state::ToastDelivery::Terminal => "\"terminal\"",
        herdr_state::ToastDelivery::System => "\"system\"",
    };
    let content = upsert_section_value(content, "ui.toast", "delivery", value);
    remove_section_key(&content, "ui.toast", "enabled")
}

pub fn config_update_agent_border_labels(content: &str, enabled: bool) -> String {
    upsert_section_bool(content, "ui", "show_agent_labels_on_pane_borders", enabled)
}

pub fn config_update_pane_history_persistence(content: &str, enabled: bool) -> String {
    upsert_section_bool(content, "experimental", "pane_history", enabled)
}

pub fn config_update_switch_ascii_input_source(content: &str, enabled: bool) -> String {
    upsert_section_bool(
        content,
        "experimental",
        "switch_ascii_input_source_in_prefix",
        enabled,
    )
}

pub fn config_update_agent_panel_sort(content: &str, sort: herdr_state::AgentPanelSort) -> String {
    let value = match sort {
        herdr_state::AgentPanelSort::Spaces => "spaces",
        herdr_state::AgentPanelSort::Priority => "priority",
    };
    upsert_section_value(content, "ui", "agent_panel_sort", &format!("\"{value}\""))
}

// ---------------------------------------------------------------------------
// TOML string manipulation helpers (moved from src/config/io.rs)
// ---------------------------------------------------------------------------

pub fn upsert_top_level_bool(content: &str, key: &str, value: bool) -> String {
    let key_value = format!("{key} = {}", if value { "true" } else { "false" });
    let first_section = content.find("\n[");
    match first_section {
        Some(pos) => {
            let before = &content[..pos];
            if before.contains(&format!("\n{key} ")) || before.contains(&format!("\n{key}=")) {
                let mut out = String::new();
                for line in content.lines() {
                    if line.starts_with(&format!("{key} "))
                        || line.starts_with(&format!("{key}="))
                    {
                        out.push_str(&key_value);
                    } else {
                        out.push_str(line);
                    }
                    out.push('\n');
                }
                out
            } else {
                format!("{key_value}\n{content}")
            }
        }
        None => {
            if content.contains(&format!("\n{key} ")) || content.contains(&format!("\n{key}=")) {
                let mut out = String::new();
                for line in content.lines() {
                    if line.starts_with(&format!("{key} "))
                        || line.starts_with(&format!("{key}="))
                    {
                        out.push_str(&key_value);
                    } else {
                        out.push_str(line);
                    }
                    out.push('\n');
                }
                out
            } else {
                format!("{key_value}\n{content}")
            }
        }
    }
}

pub fn upsert_section_value(content: &str, section: &str, key: &str, value: &str) -> String {
    upsert_scoped_key(content, section, key, value)
}

pub fn upsert_section_bool(content: &str, section: &str, key: &str, value: bool) -> String {
    let value_str = if value { "true" } else { "false" };
    upsert_scoped_key(content, section, key, value_str)
}

pub fn remove_section_key(content: &str, section: &str, key: &str) -> String {
    let section_header = format!("\n[{section}]");
    let key_prefix = format!("\n{key} ");
    let key_prefix_eq = format!("\n{key}=");

    let mut out = String::new();
    let mut in_section = false;
    for line in content.lines() {
        if line.starts_with('[') {
            in_section = line == section_header.trim_start()
                || line == section_header
                || line.starts_with(&format!("[{section}]"));
        }
        if in_section
            && (line.starts_with(&format!("{key} "))
                || line.starts_with(&format!("{key}=")))
        {
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

fn upsert_scoped_key(content: &str, section: &str, key: &str, value: &str) -> String {
    let section_header = format!("\n[{section}]");
    let key_line = format!("{key} = {value}");

    // Find the section
    let section_start = content.find(&section_header);
    let section_start = section_start.map(|pos| pos + section_header.len()).or_else(|| {
        // Section doesn't exist — append it
        let mut out = String::new();
        if !content.is_empty() && !content.ends_with('\n') {
            out.push('\n');
        }
        out.push_str(&format!("[{section}]\n{key_line}\n"));
        return Some(out.len() - 1); // placeholder, we'll just return the constructed string
    });

    match section_start {
        Some(pos) if pos < content.len() => {
            let after_section = &content[pos..];
            let next_section = after_section[1..].find("\n[").map(|p| p + 1);
            let (section_body, rest) = match next_section {
                Some(end) => (&after_section[..end], &after_section[end..]),
                None => (after_section, ""),
            };

            let mut out = String::new();
            out.push_str(&content[..pos]);
            let mut replaced = false;
            for line in section_body.lines() {
                if line.starts_with(&format!("{key} ")) || line.starts_with(&format!("{key}=")) {
                    out.push_str(&key_line);
                    replaced = true;
                } else {
                    out.push_str(line);
                }
                out.push('\n');
            }
            if !replaced {
                out.push_str(&key_line);
                out.push('\n');
            }
            out.push_str(rest);
            out
        }
        _ => {
            // Append section at end
            let mut out = content.to_string();
            if !out.is_empty() && !out.ends_with('\n') {
                out.push('\n');
            }
            out.push_str(&format!("[{section}]\n{key_line}\n"));
            out
        }
    }
}
