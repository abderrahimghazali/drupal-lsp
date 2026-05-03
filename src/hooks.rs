use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;
use walkdir::WalkDir;

#[derive(Debug, Clone)]
pub struct Hook {
    pub name: String,
    pub params: String,
    pub description: String,
}

pub fn scan_workspace(roots: &[std::path::PathBuf]) -> (HashMap<String, Hook>, usize) {
    let mut all = HashMap::new();
    let mut files_scanned = 0;

    for root in roots {
        for entry in WalkDir::new(root)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                !matches!(name.as_ref(), ".git" | "node_modules")
            })
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if !name.ends_with(".api.php") {
                continue;
            }
            let Ok(content) = fs::read_to_string(path) else {
                continue;
            };
            files_scanned += 1;
            for hook in extract(&content) {
                all.entry(hook.name.clone()).or_insert(hook);
            }
        }
    }

    (all, files_scanned)
}

fn hook_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?ms)(?:/\*\*(.*?)\*/\s*)?function\s+(hook_\w+)\s*\(([^)]*)\)").unwrap()
    })
}

pub fn extract(content: &str) -> Vec<Hook> {
    hook_regex()
        .captures_iter(content)
        .map(|cap| {
            let phpdoc = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let name = cap[2].to_string();
            let params = cap[3].split_whitespace().collect::<Vec<_>>().join(" ");
            let description = phpdoc_summary(phpdoc);
            Hook { name, params, description }
        })
        .collect()
}

fn phpdoc_summary(phpdoc: &str) -> String {
    phpdoc
        .lines()
        .map(|l| l.trim().trim_start_matches('*').trim())
        .find(|l| !l.is_empty() && !l.starts_with('@'))
        .unwrap_or("")
        .to_string()
}

#[allow(dead_code)]
pub fn looks_like_drupal_root(path: &Path) -> bool {
    path.join("composer.json").exists() || path.join("core").is_dir()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_hook_with_phpdoc() {
        let src = r#"
/**
 * Provide help text.
 *
 * @param string $route_name
 * @return string
 */
function hook_help($route_name, RouteMatchInterface $route_match) {
}
"#;
        let hooks = extract(src);
        assert_eq!(hooks.len(), 1);
        assert_eq!(hooks[0].name, "hook_help");
        assert_eq!(hooks[0].description, "Provide help text.");
        assert!(hooks[0].params.contains("$route_name"));
    }

    #[test]
    fn extracts_hook_without_phpdoc() {
        let src = "function hook_form_alter(&$form, $form_state, $form_id) {}";
        let hooks = extract(src);
        assert_eq!(hooks.len(), 1);
        assert_eq!(hooks[0].name, "hook_form_alter");
    }

    #[test]
    fn skips_non_hook_functions() {
        let src = "function some_helper() {} function hook_x() {}";
        let hooks = extract(src);
        assert_eq!(hooks.len(), 1);
        assert_eq!(hooks[0].name, "hook_x");
    }
}
