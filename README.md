# mdbook-mermaid-mmdr

An `mdbook` preprocessor that renders [Mermaid](https://mermaid-js.github.io/mermaid/#/) diagrams server-side using the Rust-based [mermaid-rs-renderer](https://github.com/1jehuang/mermaid-rs-renderer).

This preprocessor generates SVG diagrams at build time, eliminating the need for client-side JavaScript libraries or headless browser dependencies like Puppeteer.

## Features

- **Server-Side Rendering**: Generates static SVG diagrams during `mdbook build`.
- **Zero Client-Side JS**: No heavy JavaScript bundles or runtime rendering.
- **Pure Rust**: No Node.js, npm, or Puppeteer required.
- **Configurable**: Supports Mermaid themes and background customization.
- **Robust**: Inline error reporting prevents build failures on invalid diagrams.

## Installation

To build and install from source:

```bash
cargo install --path .
```

Ensure the binary is in your `PATH`.

## Usage

Add the preprocessor to your `book.toml`:

```toml
[preprocessor.mermaid-mmdr]
command = "mdbook-mermaid-mmdr" # Optional if in PATH
```

Then, simply use `mermaid` code blocks in your markdown files:

```markdown
```mermaid
graph TD;
    A-->B;
    A-->C;
    B-->D;
    C-->D;
```
```

## Configuration

You can configure the theme and background color in `book.toml`:

```toml
[preprocessor.mermaid-mmdr]
# Theme options: "default", "forest", "neutral", "dark", "base".
# If not specified, defaults to "modern".
theme = "neutral"

# Background color for the diagram. Defaults to transparent.
background = "#ffffff"
```

## Examples

This repository includes several example books in the `examples/` directory:

-   `comprehensive`: Demonstrates all supported diagram types and customization.
-   `simple`: A minimal example with basic diagrams.
-   `themed`: Shows how to apply different Mermaid themes (e.g., `forest`).

To build an example:

1.  Build the preprocessor:
    ```bash
    cargo build
    ```
2.  Navigate to the desired example directory (e.g., `examples/comprehensive`):
    ```bash
    cd examples/comprehensive
    ```
3.  Build the book:
    ```bash
    mdbook build
    ```
4.  Open `book/index.html` in your browser.

## License

[MIT](LICENSE)
