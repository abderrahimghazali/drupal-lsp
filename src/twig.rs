use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, InsertTextFormat};

pub struct TwigItem {
    pub label: &'static str,
    pub detail: &'static str,
    pub snippet: Option<&'static str>,
}

pub const TAGS: &[TwigItem] = &[
    TwigItem { label: "if", detail: "Conditional block", snippet: Some("if ${1:condition} %}\n  $0\n{% endif") },
    TwigItem { label: "for", detail: "Loop over a sequence", snippet: Some("for ${1:item} in ${2:items} %}\n  $0\n{% endfor") },
    TwigItem { label: "block", detail: "Define a block", snippet: Some("block ${1:name} %}\n  $0\n{% endblock") },
    TwigItem { label: "extends", detail: "Extend a template", snippet: Some("extends '${1:template.html.twig}' %}") },
    TwigItem { label: "include", detail: "Include another template", snippet: Some("include '${1:template.html.twig}' %}") },
    TwigItem { label: "embed", detail: "Embed another template", snippet: Some("embed '${1:template.html.twig}' %}\n  $0\n{% endembed") },
    TwigItem { label: "set", detail: "Assign a variable", snippet: Some("set ${1:name} = ${2:value} %}") },
    TwigItem { label: "macro", detail: "Define a macro", snippet: Some("macro ${1:name}(${2:args}) %}\n  $0\n{% endmacro") },
    TwigItem { label: "import", detail: "Import macros", snippet: Some("import '${1:template.html.twig}' as ${2:alias} %}") },
    TwigItem { label: "from", detail: "Import specific macros", snippet: Some("from '${1:template.html.twig}' import ${2:macro} %}") },
    TwigItem { label: "use", detail: "Inherit blocks horizontally", snippet: Some("use '${1:template.html.twig}' %}") },
    TwigItem { label: "spaceless", detail: "Strip whitespace between tags", snippet: Some("spaceless %}\n  $0\n{% endspaceless") },
    TwigItem { label: "verbatim", detail: "Raw output, no parsing", snippet: Some("verbatim %}\n  $0\n{% endverbatim") },
    TwigItem { label: "with", detail: "Limited variable scope", snippet: Some("with { ${1:key}: ${2:value} } %}\n  $0\n{% endwith") },
    TwigItem { label: "trans", detail: "Drupal translation block", snippet: Some("trans %}\n  $0\n{% endtrans") },
    TwigItem { label: "apply", detail: "Apply filter to block", snippet: Some("apply ${1:filter} %}\n  $0\n{% endapply") },
    TwigItem { label: "do", detail: "Run expression without output", snippet: Some("do ${1:expression} %}") },
    TwigItem { label: "else", detail: "Else branch", snippet: None },
    TwigItem { label: "elseif", detail: "Else-if branch", snippet: Some("elseif ${1:condition} %}") },
    TwigItem { label: "endif", detail: "End if block", snippet: None },
    TwigItem { label: "endfor", detail: "End for block", snippet: None },
    TwigItem { label: "endblock", detail: "End block", snippet: None },
    TwigItem { label: "endembed", detail: "End embed block", snippet: None },
    TwigItem { label: "endmacro", detail: "End macro block", snippet: None },
    TwigItem { label: "endspaceless", detail: "End spaceless block", snippet: None },
    TwigItem { label: "endverbatim", detail: "End verbatim block", snippet: None },
    TwigItem { label: "endtrans", detail: "End translation block", snippet: None },
    TwigItem { label: "endapply", detail: "End apply block", snippet: None },
    TwigItem { label: "endwith", detail: "End with block", snippet: None },
];

pub const FILTERS: &[TwigItem] = &[
    TwigItem { label: "t", detail: "Drupal: translate string", snippet: None },
    TwigItem { label: "trans", detail: "Drupal: translate string", snippet: None },
    TwigItem { label: "safe_join", detail: "Drupal: join values escaping each", snippet: Some("safe_join('${1:, }')") },
    TwigItem { label: "clean_class", detail: "Drupal: produce a valid CSS class", snippet: None },
    TwigItem { label: "clean_id", detail: "Drupal: produce a valid HTML id", snippet: None },
    TwigItem { label: "render", detail: "Drupal: render a render array", snippet: None },
    TwigItem { label: "placeholder", detail: "Drupal: format as placeholder", snippet: None },
    TwigItem { label: "without", detail: "Drupal: render array minus keys", snippet: Some("without('${1:key}')") },
    TwigItem { label: "add_class", detail: "Drupal: add HTML classes (Attribute)", snippet: Some("add_class('${1:class}')") },
    TwigItem { label: "remove_class", detail: "Drupal: remove HTML classes (Attribute)", snippet: Some("remove_class('${1:class}')") },
    TwigItem { label: "set_attribute", detail: "Drupal: set HTML attribute", snippet: Some("set_attribute('${1:name}', '${2:value}')") },
    TwigItem { label: "format_date", detail: "Drupal: format a timestamp", snippet: Some("format_date('${1:medium}')") },
    TwigItem { label: "abs", detail: "Absolute value", snippet: None },
    TwigItem { label: "batch", detail: "Group items into batches", snippet: Some("batch(${1:size})") },
    TwigItem { label: "capitalize", detail: "Capitalize a string", snippet: None },
    TwigItem { label: "column", detail: "Return column from array", snippet: Some("column(${1:name})") },
    TwigItem { label: "convert_encoding", detail: "Convert charset", snippet: Some("convert_encoding('${1:UTF-8}', '${2:ISO-8859-1}')") },
    TwigItem { label: "country_name", detail: "Localized country name", snippet: None },
    TwigItem { label: "currency_name", detail: "Localized currency name", snippet: None },
    TwigItem { label: "currency_symbol", detail: "Localized currency symbol", snippet: None },
    TwigItem { label: "data_uri", detail: "Generate a data URI", snippet: None },
    TwigItem { label: "date", detail: "Format a date", snippet: Some("date('${1:Y-m-d}')") },
    TwigItem { label: "date_modify", detail: "Modify a date", snippet: Some("date_modify('${1:+1 day}')") },
    TwigItem { label: "default", detail: "Default value if empty", snippet: Some("default('${1:fallback}')") },
    TwigItem { label: "escape", detail: "Escape a string", snippet: None },
    TwigItem { label: "e", detail: "Escape (alias)", snippet: None },
    TwigItem { label: "filter", detail: "Filter items", snippet: Some("filter(${1:v} => ${2:condition})") },
    TwigItem { label: "first", detail: "First item", snippet: None },
    TwigItem { label: "format", detail: "Format with sprintf", snippet: Some("format(${1:args})") },
    TwigItem { label: "format_currency", detail: "Format as currency", snippet: Some("format_currency('${1:USD}')") },
    TwigItem { label: "format_number", detail: "Format a number", snippet: None },
    TwigItem { label: "join", detail: "Join values with a separator", snippet: Some("join('${1:, }')") },
    TwigItem { label: "json_encode", detail: "Encode as JSON", snippet: None },
    TwigItem { label: "keys", detail: "Array keys", snippet: None },
    TwigItem { label: "language_name", detail: "Localized language name", snippet: None },
    TwigItem { label: "last", detail: "Last item", snippet: None },
    TwigItem { label: "length", detail: "Length / count", snippet: None },
    TwigItem { label: "locale_name", detail: "Localized locale name", snippet: None },
    TwigItem { label: "lower", detail: "Lowercase", snippet: None },
    TwigItem { label: "map", detail: "Apply mapping", snippet: Some("map(${1:v} => ${2:expr})") },
    TwigItem { label: "merge", detail: "Merge arrays", snippet: Some("merge(${1:other})") },
    TwigItem { label: "nl2br", detail: "Newlines to <br>", snippet: None },
    TwigItem { label: "number_format", detail: "Format a number", snippet: Some("number_format(${1:2}, '${2:.}', '${3:,}')") },
    TwigItem { label: "raw", detail: "Mark as safe; do not escape", snippet: None },
    TwigItem { label: "reduce", detail: "Reduce to single value", snippet: Some("reduce((${1:acc}, ${2:v}) => ${3:expr}, ${4:initial})") },
    TwigItem { label: "replace", detail: "String replacement", snippet: Some("replace({ '${1:from}': '${2:to}' })") },
    TwigItem { label: "reverse", detail: "Reverse sequence", snippet: None },
    TwigItem { label: "round", detail: "Round a number", snippet: None },
    TwigItem { label: "slice", detail: "Slice a sequence", snippet: Some("slice(${1:start}, ${2:length})") },
    TwigItem { label: "sort", detail: "Sort items", snippet: None },
    TwigItem { label: "spaceless", detail: "Remove whitespace between tags", snippet: None },
    TwigItem { label: "split", detail: "Split a string", snippet: Some("split('${1:,}')") },
    TwigItem { label: "striptags", detail: "Strip HTML tags", snippet: None },
    TwigItem { label: "timezone_name", detail: "Localized timezone name", snippet: None },
    TwigItem { label: "title", detail: "Title case", snippet: None },
    TwigItem { label: "trim", detail: "Trim whitespace", snippet: None },
    TwigItem { label: "u", detail: "Unicode string operations", snippet: None },
    TwigItem { label: "upper", detail: "Uppercase", snippet: None },
    TwigItem { label: "url_encode", detail: "URL encode", snippet: None },
];

pub const FUNCTIONS: &[TwigItem] = &[
    TwigItem { label: "url", detail: "Drupal: generate an absolute URL", snippet: Some("url('${1:route_name}')") },
    TwigItem { label: "path", detail: "Drupal: generate an internal path", snippet: Some("path('${1:route_name}')") },
    TwigItem { label: "link", detail: "Drupal: render a link", snippet: Some("link('${1:text}', '${2:url}')") },
    TwigItem { label: "file_url", detail: "Drupal: URL for a public file", snippet: Some("file_url('${1:public://file.png}')") },
    TwigItem { label: "attach_library", detail: "Drupal: attach an asset library", snippet: Some("attach_library('${1:module/library}')") },
    TwigItem { label: "active_theme", detail: "Drupal: name of the active theme", snippet: None },
    TwigItem { label: "active_theme_path", detail: "Drupal: path to the active theme", snippet: None },
    TwigItem { label: "create_attribute", detail: "Drupal: create an Attribute object", snippet: Some("create_attribute({ ${1:key}: '${2:value}' })") },
    TwigItem { label: "render_var", detail: "Drupal: render any variable", snippet: Some("render_var(${1:var})") },
    TwigItem { label: "block", detail: "Output a block from a parent template", snippet: Some("block('${1:name}')") },
    TwigItem { label: "parent", detail: "Output the parent block content", snippet: None },
    TwigItem { label: "constant", detail: "Read a PHP constant", snippet: Some("constant('${1:NAME}')") },
    TwigItem { label: "cycle", detail: "Cycle through values", snippet: Some("cycle([${1:'a', 'b'}], ${2:i})") },
    TwigItem { label: "date", detail: "Convert to a date object", snippet: None },
    TwigItem { label: "dump", detail: "Dump variables for debugging", snippet: None },
    TwigItem { label: "html_classes", detail: "Build a class string conditionally", snippet: Some("html_classes(${1:'class', condition ? 'other'})") },
    TwigItem { label: "include", detail: "Include another template (function)", snippet: Some("include('${1:template.html.twig}')") },
    TwigItem { label: "max", detail: "Maximum value", snippet: None },
    TwigItem { label: "min", detail: "Minimum value", snippet: None },
    TwigItem { label: "random", detail: "Random value or array element", snippet: None },
    TwigItem { label: "range", detail: "Sequence of numbers", snippet: Some("range(${1:1}, ${2:10})") },
    TwigItem { label: "source", detail: "Read template source verbatim", snippet: Some("source('${1:template.html.twig}')") },
    TwigItem { label: "template_from_string", detail: "Compile template from string", snippet: Some("template_from_string('${1:source}')") },
];

#[derive(Debug, PartialEq, Eq)]
pub enum Context {
    Outside,
    Statement,
    Expression,
    AfterPipe,
}

pub fn detect_context(line_before_cursor: &str) -> Context {
    let last_open_stmt = line_before_cursor.rfind("{%");
    let last_close_stmt = line_before_cursor.rfind("%}");
    let last_open_expr = line_before_cursor.rfind("{{");
    let last_close_expr = line_before_cursor.rfind("}}");
    let last_pipe = line_before_cursor.rfind('|');

    let in_stmt = match (last_open_stmt, last_close_stmt) {
        (Some(o), Some(c)) => o > c,
        (Some(_), None) => true,
        _ => false,
    };
    let in_expr = match (last_open_expr, last_close_expr) {
        (Some(o), Some(c)) => o > c,
        (Some(_), None) => true,
        _ => false,
    };

    if !in_stmt && !in_expr {
        return Context::Outside;
    }

    let region_start = if in_stmt { last_open_stmt } else { last_open_expr };
    if let (Some(pipe), Some(start)) = (last_pipe, region_start) {
        if pipe > start {
            let after = &line_before_cursor[pipe + 1..];
            if !after.contains('|') && after.chars().all(|c| c.is_alphanumeric() || c == '_') {
                return Context::AfterPipe;
            }
        }
    }

    if in_stmt {
        Context::Statement
    } else {
        Context::Expression
    }
}

pub fn completions(line_before_cursor: &str) -> Vec<CompletionItem> {
    match detect_context(line_before_cursor) {
        Context::Outside => Vec::new(),
        Context::AfterPipe => FILTERS.iter().map(|f| to_item(f, CompletionItemKind::FUNCTION)).collect(),
        Context::Statement => TAGS.iter().map(|t| to_item(t, CompletionItemKind::KEYWORD)).collect(),
        Context::Expression => {
            let mut items: Vec<CompletionItem> =
                FUNCTIONS.iter().map(|f| to_item(f, CompletionItemKind::FUNCTION)).collect();
            items.extend(FILTERS.iter().map(|f| to_item(f, CompletionItemKind::FUNCTION)));
            items
        }
    }
}

fn to_item(item: &TwigItem, kind: CompletionItemKind) -> CompletionItem {
    let (insert_text, format) = match item.snippet {
        Some(s) => (s.to_string(), Some(InsertTextFormat::SNIPPET)),
        None => (item.label.to_string(), None),
    };
    CompletionItem {
        label: item.label.into(),
        kind: Some(kind),
        detail: Some(item.detail.into()),
        insert_text: Some(insert_text),
        insert_text_format: format,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outside_when_no_delimiters() {
        assert_eq!(detect_context("hello world"), Context::Outside);
    }

    #[test]
    fn statement_inside_curly_percent() {
        assert_eq!(detect_context("  {% i"), Context::Statement);
    }

    #[test]
    fn expression_inside_double_curly() {
        assert_eq!(detect_context("hello {{ user."), Context::Expression);
    }

    #[test]
    fn after_pipe_in_expression() {
        assert_eq!(detect_context("{{ name|"), Context::AfterPipe);
        assert_eq!(detect_context("{{ name|t"), Context::AfterPipe);
    }

    #[test]
    fn closed_delimiters_return_outside() {
        assert_eq!(detect_context("{{ name }} "), Context::Outside);
        assert_eq!(detect_context("{% if true %} body "), Context::Outside);
    }

    #[test]
    fn statement_completions_contain_tags() {
        let items = completions("{% i");
        assert!(items.iter().any(|i| i.label == "if"));
        assert!(items.iter().any(|i| i.label == "for"));
    }

    #[test]
    fn expression_completions_contain_functions() {
        let items = completions("{{ ");
        assert!(items.iter().any(|i| i.label == "path"));
        assert!(items.iter().any(|i| i.label == "url"));
    }

    #[test]
    fn after_pipe_completions_are_filters_only() {
        let items = completions("{{ name|");
        assert!(items.iter().any(|i| i.label == "t"));
        assert!(!items.iter().any(|i| i.label == "if"));
    }
}
