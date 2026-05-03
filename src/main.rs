mod hooks;
mod twig;

use std::collections::HashMap;
use std::path::PathBuf;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::hooks::Hook;

#[derive(Debug)]
struct Backend {
    client: Client,
    workspace_roots: RwLock<Vec<PathBuf>>,
    hooks: RwLock<HashMap<String, Hook>>,
    documents: RwLock<HashMap<Url, String>>,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            workspace_roots: RwLock::new(Vec::new()),
            hooks: RwLock::new(HashMap::new()),
            documents: RwLock::new(HashMap::new()),
        }
    }

    async fn rescan(&self) {
        let roots = self.workspace_roots.read().await.clone();
        let (all_hooks, files_scanned) = hooks::scan_workspace(&roots);
        let count = all_hooks.len();
        *self.hooks.write().await = all_hooks;
        self.client
            .log_message(
                MessageType::INFO,
                format!("drupal-lsp: indexed {count} hooks from {files_scanned} *.api.php files"),
            )
            .await;
    }

    async fn hook_completions(&self) -> Vec<CompletionItem> {
        let hooks = self.hooks.read().await;
        if hooks.is_empty() {
            return vec![CompletionItem {
                label: "hook_help".into(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("No *.api.php files indexed in workspace".into()),
                insert_text: Some("hook_help".into()),
                ..Default::default()
            }];
        }
        hooks
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
            .collect()
    }

    async fn line_before_cursor(&self, uri: &Url, position: Position) -> Option<String> {
        let docs = self.documents.read().await;
        let text = docs.get(uri)?;
        let line = text.lines().nth(position.line as usize)?;
        let end = (position.character as usize).min(line.len());
        Some(line[..end].to_string())
    }
}

fn is_twig_uri(uri: &Url) -> bool {
    uri.path().ends_with(".twig")
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
                    trigger_characters: Some(vec![
                        "_".into(),
                        "|".into(),
                        "{".into(),
                        "%".into(),
                    ]),
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
        self.rescan().await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.documents
            .write()
            .await
            .insert(params.text_document.uri, params.text_document.text);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Some(change) = params.content_changes.into_iter().next_back() {
            self.documents
                .write()
                .await
                .insert(params.text_document.uri, change.text);
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.write().await.remove(&params.text_document.uri);
    }

    async fn completion(
        &self,
        params: CompletionParams,
    ) -> Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        if is_twig_uri(&uri) {
            let line = self
                .line_before_cursor(&uri, position)
                .await
                .unwrap_or_default();
            let items = twig::completions(&line);
            return Ok(Some(CompletionResponse::Array(items)));
        }

        Ok(Some(CompletionResponse::Array(self.hook_completions().await)))
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    let (service, socket) = LspService::new(Backend::new);
    Server::new(stdin, stdout, socket).serve(service).await;
}
