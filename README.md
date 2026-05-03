# drupal-lsp

A Language Server Protocol implementation for Drupal development.

Powers the [zed-drupal](https://github.com/abderrahimghazali/zed-drupal)
extension. Should also work with any LSP-capable editor (Neovim, Helix,
VS Code via a generic client).

## Status

Pre-alpha. Currently returns one hardcoded completion item to verify the
plumbing works.

## Planned

- Hook completion (parses `*.api.php`, `.module`, `.theme`)
- Twig template completion
- Service container completion (`*.services.yml`)
- Routing completion (`*.routing.yml`)
- Translation autocomplete (PHP `t()`, Twig `{% trans %}`, JS `Drupal.t()`)
- Global variables completion
- PHPCS diagnostics (`drupal/coder` rules)
- PHPCBF formatting
- PHPStan diagnostics with `phpstan-drupal`
- YAML schema validation for Drupal config files

## Build

```sh
cargo build --release
# Binary lands at target/release/drupal-lsp
```

## License

Apache 2.0.
