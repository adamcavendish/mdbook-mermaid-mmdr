# Getting Started

## Installation

### From source

```bash
cargo install --path .
```

### From crates.io

```bash
cargo install mdbook-mermaid-mmdr
```

Make sure the installed binary is on your `PATH`.

### Linux dependency

On Linux, the font/SVG rendering stack requires fontconfig development headers:

```bash
sudo apt-get install -y libfontconfig1-dev
```

## Setup

Add the preprocessor to your `book.toml`:

```toml
[preprocessor.mermaid-mmdr]
command = "mdbook-mermaid-mmdr"
```

The `command` field is optional if the binary is already in your `PATH` and named `mdbook-mermaid-mmdr`.

## Writing Mermaid Diagrams

Use standard fenced code blocks with the `mermaid` language tag:

````markdown
```mermaid
graph TD;
    A-->B;
    A-->C;
    B-->D;
    C-->D;
```
````

When you run `mdbook build`, the code block is replaced with an inline SVG.

## Running the Examples

The repository includes example books under `examples/`:

| Example | Description |
|---|---|
| `simple` | A minimal book with a single flowchart |
| `comprehensive` | All supported diagram types and customization |
| `themed` | Custom theme variables via `book.toml` |

To build an example:

```bash
cargo build
cd examples/simple
mdbook build
open book/index.html
```
