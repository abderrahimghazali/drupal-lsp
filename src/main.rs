use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
struct Hook {
    name: String,
    params: String,
    description: String,
}

#[derive(Debug)]
struct Backend {
    client: Client,
    workspace_roots: RwLock<Vec<PathBuf>>,
    hooks: RwLock<HashMap<String, Hook>>,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            workspace_roots: RwLock::new(Vec::new()),
            hooks: RwLock::new(HashMap::new()),
        }
    }

    async fn scan_workspace(&self) {
        let roots = self.workspace_roots.read().await.clone();
        let mut all_hooks: HashMap<String, Hook> = HashMap::new();
        let mut files_scanned = 0usize;

        for root in &roots {
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
                for hook in extract_hooks(&content) {
                    all_hooks.entry(hook.name.clone()).or_insert(hook);
                }
            }
        }

        let count = all_hooks.len();
        *self.hooks.write().await = all_hooks;
        self.client
            .log_message(
                MessageType::INFO,
                format!("drupal-lsp: indexed {count} hooks from {files_scanned} *.api.php files"),
            )
            .await;
    }
}

fn hook_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?ms)(?:/\*\*(.*?)\*/\s*)?function\s+(hook_\w+)\s*\(([^)]*)\)").unwrap()
    })
}

fn extract_hooks(content: &str) -> Vec<Hook> {
    hook_regex()
        .captures_iter(content)
        .map(|cap| {
            let phpdoc = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let name = cap[2].to_string();
            let params = cap[3].split_whitespace().collect::<Vec<_>>().join(" ");
            let description = extract_phpdoc_summary(phpdoc);
            Hook { name, params, description }
        })
        .collect()
}

fn extract_phpdoc_summary(phpdoc: &str) -> String {
    phpdoc
        .lines()
        .map(|l| l.trim().trim_start_matches('*').trim())
        .find(|l| !l.is_empty() && !l.starts_with('@'))
        .unwrap_or("")
        .to_string()
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        let mut roots = Vec::new();
        if let Some(folders) = params.workspace_folders {
            for f in folders {
                if let Ok(path) = f.uri.to_file_path() {
                    roots.push(path);
                }
            }
        }
        #[allow(deprecated)]
        if roots.is_empty() {
            if let Some(uri) = params.root_uri {
                if let Ok(path) = uri.to_file_path() {
                    roots.push(path);
                }
            }
        }
        *self.workspace_roots.write().await = roots;

        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "drupal-lsp".into(),
                version: Some(env!("CARGO_PKG_VERSION").into()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec!["_".into()]),
                    ..Default::default()
                }),
                ..Default::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "drupal-lsp ready, scanning workspace")
            .await;
        self.scan_workspace().await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn completion(
        &self,
        _params: CompletionParams,
    ) -> Result<Option<CompletionResponse>> {
        let hooks = self.hooks.read().await;

        if hooks.is_empty() {
            return Ok(Some(CompletionResponse::Array(vec![CompletionItem {
                label: "hook_help".into(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("No *.api.php files indexed in workspace".into()),
                insert_text: Some("hook_help".into()),
                ..Default::default()
            }])));
        }

        let items: Vec<CompletionItem> = hooks
            .values()
            .map(|h| {
                let suffix = h.name.strip_prefix("hook_").unwrap_or(&h.name);
                let snippet = format!(
                    "function ${{1:my_module}}_{}({}) {{\n  $0\n}}",
                    suffix, h.params
                );
                CompletionItem {
                    label: h.name.clone(),
                    kind: Some(CompletionItemKind::FUNCTION),
                    detail: if h.description.is_empty() {
                        Some("Drupal hook".into())
                    } else {
                        Some(h.description.clone())
                    },
                    insert_text: Some(snippet),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                }
            })
            .collect();
        Ok(Some(CompletionResponse::Array(items)))
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
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
        let hooks = extract_hooks(src);
        assert_eq!(hooks.len(), 1);
        assert_eq!(hooks[0].name, "hook_help");
        assert_eq!(hooks[0].description, "Provide help text.");
        assert!(hooks[0].params.contains("$route_name"));
    }

    #[test]
    fn extracts_hook_without_phpdoc() {
        let src = "function hook_form_alter(&$form, $form_state, $form_id) {}";
        let hooks = extract_hooks(src);
        assert_eq!(hooks.len(), 1);
        assert_eq!(hooks[0].name, "hook_form_alter");
    }

    #[test]
    fn skips_non_hook_functions() {
        let src = "function some_helper() {} function hook_x() {}";
        let hooks = extract_hooks(src);
        assert_eq!(hooks.len(), 1);
        assert_eq!(hooks[0].name, "hook_x");
    }
}
