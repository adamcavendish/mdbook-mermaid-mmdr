use log::warn;
use mermaid_rs_renderer::{render_with_options, RenderOptions, Theme};
use serde_json::Value;

use crate::config::Config;

pub fn render(code: &str, config: &Config) -> String {
    let mut options = RenderOptions::default();

    if let Some(theme_name) = &config.theme {
        match theme_name.as_str() {
            "default" | "mermaid" => options.theme = Theme::mermaid_default(),
            "modern" => options.theme = Theme::modern(),
            _ => options.theme = Theme::modern(),
        }
    }

    if let Some(bg) = &config.background {
        options.theme.background = bg.clone();
    }

    if let Some(theme_variables) = &config.theme_variables {
        match serde_json::to_value(&options.theme) {
            Ok(Value::Object(mut map)) => {
                if let Value::Object(overrides) = theme_variables {
                    for (k, v) in overrides {
                        map.insert(k.clone(), v.clone());
                    }
                    match serde_json::from_value(Value::Object(map)) {
                        Ok(theme) => options.theme = theme,
                        Err(e) => warn!("Failed to apply theme variables: {}", e),
                    }
                }
            }
            Ok(_) => warn!("Failed to serialize theme to object"),
            Err(e) => warn!("Failed to serialize theme: {}", e),
        }
    }

    match render_with_options(code, options) {
        Ok(svg) => {
            // Fix nested quotes in font-family attribute produced by mermaid-rs-renderer
            fix_nested_quotes(svg)
        }
        Err(e) => {
            warn!("Mermaid rendering failed: {}", e);
            format!("<pre class=\"mermaid-error\">Error: {}</pre>", e)
        }
    }
}

fn fix_nested_quotes(svg: String) -> String {
    let marker = "font-family=\"";
    let mut result = String::with_capacity(svg.len());
    let mut last_pos = 0;

    while let Some(pos) = svg[last_pos..].find(marker) {
        let abs_pos = last_pos + pos;
        let content_start = abs_pos + marker.len();

        result.push_str(&svg[last_pos..content_start]);

        let rest = &svg[content_start..];
        let mut end_found = false;
        let mut content_end = 0;

        for (i, c) in rest.char_indices() {
            if c == '"' {
                let next_char = rest[i + 1..].chars().next();
                let is_closing = match next_char {
                    Some(nc) => matches!(nc, ' ' | '>' | '/' | '\t' | '\n' | '\r'),
                    None => true,
                };

                if is_closing {
                    content_end = i;
                    end_found = true;
                    break;
                }
            }
        }

        if end_found {
            let content = &rest[..content_end];
            let fixed_content = content.replace("\"", "'");
            result.push_str(&fixed_content);
            result.push('"');
            last_pos = content_start + content_end + 1;
        } else {
            last_pos = content_start;
            break;
        }
    }

    result.push_str(&svg[last_pos..]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_nested_quotes_standard() {
        let input = r#"<svg ... font-family="Inter, ui-sans-serif, system-ui, -apple-system, "Segoe UI", sans-serif" ...>"#;
        let expected = r#"<svg ... font-family="Inter, ui-sans-serif, system-ui, -apple-system, 'Segoe UI', sans-serif" ...>"#;
        assert_eq!(fix_nested_quotes(input.to_string()), expected);
    }

    #[test]
    fn test_fix_nested_quotes_variations() {
        let input = r#"<svg ... font-family="Arial, "Helvetica Neue", sans-serif" ...>"#;
        let expected = r#"<svg ... font-family="Arial, 'Helvetica Neue', sans-serif" ...>"#;
        assert_eq!(fix_nested_quotes(input.to_string()), expected);
    }
}
