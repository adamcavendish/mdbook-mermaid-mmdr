# Error Handling

mdbook-mermaid-mmdr is designed to never fail the build, even when a diagram is invalid.

## Inline Error Messages

When a mermaid code block fails to render, the preprocessor replaces it with an HTML error block:

```html
<pre class="mermaid-error">Error: &lt;error details&gt;</pre>
```

A warning is also logged to stderr. The rest of the book continues to build normally.

## Styling Errors

You can style error blocks in a custom mdbook theme:

```css
.mermaid-error {
    color: #cc0000;
    background: #fff0f0;
    border: 1px solid #cc0000;
    padding: 1em;
    border-radius: 4px;
}
```

## Debugging

Set the `RUST_LOG` environment variable to see detailed output:

```bash
RUST_LOG=debug mdbook build
```

The preprocessor logs theme resolution, rendering attempts, and any SVG post-processing at the `debug` and `warn` levels.
