# Configuration

All configuration lives in your `book.toml` under the `[preprocessor.mermaid-mmdr]` table. Every field is optional.

## Full Example

```toml
[preprocessor.mermaid-mmdr]
command = "mdbook-mermaid-mmdr"
theme = "modern"
background = "#ffffff"

[preprocessor.mermaid-mmdr.theme_variables]
primary_color = "#ffcccc"
line_color = "#ff0000"
secondary_color = "#ccffcc"
```

## Reference

### `theme`

The base theme for rendering. Accepted values:

| Value | Effect |
|---|---|
| `"default"` or `"mermaid"` | The classic Mermaid color scheme (`Theme::mermaid_default()`) |
| `"modern"` | A cleaner, modern palette (`Theme::modern()`) |
| *(any other string)* | Falls back to `"modern"` |
| *(omitted)* | Uses `mermaid-rs-renderer`'s default |

For fine-grained theme control beyond these two base themes, use `theme_variables` or inline `%%{init: ...}%%` directives in the diagram source.

### `background`

A CSS color string for the diagram background (e.g., `"#ffffff"`, `"transparent"`). Overrides the background from the selected theme.

### `theme_variables`

A TOML table of key-value overrides merged into the resolved theme. This lets you tweak individual colors without replacing the entire theme.

```toml
[preprocessor.mermaid-mmdr.theme_variables]
primary_color = "#4a86c8"
primary_text_color = "#ffffff"
line_color = "#333333"
```

The keys correspond to fields on `mermaid-rs-renderer`'s `Theme` struct. Unknown keys are silently ignored.
