//! A mdbook preprocessor that renders Mermaid diagrams using `mermaid-cli` (mmdc).
//!
//! This preprocessor searches for mermaid code blocks in your markdown files
//! and replaces them with SVG images rendered by `mermaid-cli`.

pub mod config;
mod error;
mod renderer;

use log::{debug, info};
use mdbook_preprocessor::book::{Book, BookItem};
use mdbook_preprocessor::errors::Error;
use mdbook_preprocessor::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag, TagEnd};

/// A preprocessor for mdbook that renders Mermaid diagrams.
pub struct MermaidPreprocessor;

impl MermaidPreprocessor {
    /// Creates a new `MermaidPreprocessor`.
    pub fn new() -> Self {
        MermaidPreprocessor
    }
}

impl Default for MermaidPreprocessor {
    fn default() -> Self {
        Self::new()
    }
}

impl Preprocessor for MermaidPreprocessor {
    fn name(&self) -> &str {
        "mermaid-mmdr"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        info!("Starting mermaid-mmdr preprocessor");
        let config = config::Config::from_context(ctx);

        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                chapter.content = process_markdown(&chapter.content, &config);
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> Result<bool, Error> {
        Ok(renderer != "not-supported")
    }
}

fn process_markdown(content: &str, config: &config::Config) -> String {
    let mut new_content = String::with_capacity(content.len());
    let mut last_end = 0;
    let mut mermaid_buffer = String::new();

    let opts = pulldown_cmark::Options::all();
    let parser = Parser::new_ext(content, opts).into_offset_iter();
    let mut in_mermaid = false;

    for (event, span) in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang)))
                if lang.as_ref() == "mermaid" =>
            {
                in_mermaid = true;
                new_content.push_str(&content[last_end..span.start]);
                mermaid_buffer.clear();
            }
            Event::End(TagEnd::CodeBlock) if in_mermaid => {
                in_mermaid = false;
                debug!("Found mermaid block, length: {}", mermaid_buffer.len());
                let svg = renderer::render(&mermaid_buffer, config);
                new_content.push_str("\n\n<div class=\"mermaid-mmdr-svg\">\n");
                new_content.push_str(&svg);
                new_content.push_str("\n</div>\n\n");
                last_end = span.end;
            }
            Event::Text(text) if in_mermaid => {
                mermaid_buffer.push_str(&text);
            }
            _ => {}
        }
    }

    new_content.push_str(&content[last_end..]);
    new_content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_markdown_with_mermaid() {
        let content = r#"
# Chapter 1

```mermaid
graph TD;
    A-->B;
    A-->C;
    B-->D;
    C-->D;
```

Some text after.
"#;
        let result = process_markdown(content, &config::Config::default());
        assert!(result.contains("<svg"));
        assert!(result.contains("Some text after."));
    }

    #[test]
    fn test_process_markdown_no_mermaid() {
        let content = r#"
# Chapter 1

```rust
fn main() {}
```
"#;
        let expected = content.to_string();
        assert_eq!(
            process_markdown(content, &config::Config::default()),
            expected
        );
    }

    #[test]
    fn test_process_markdown_multiple_mermaid() {
        let content = r#"
```mermaid
graph TD;
    A-->B;
```

Middle text.

```mermaid
graph TD;
    C-->D;
```
"#;
        let result = process_markdown(content, &config::Config::default());
        assert!(result.matches("<svg").count() == 2);
        assert!(result.contains("Middle text."));
    }
}
