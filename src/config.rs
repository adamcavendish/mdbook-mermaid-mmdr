use std::collections::BTreeMap;

use mdbook_preprocessor::PreprocessorContext;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[cfg(test)]
use mdbook_core::config::Config as MdBookConfig;

/// Configuration for the Mermaid preprocessor.
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// The theme to use for the Mermaid diagrams (e.g., "default", "dark", "forest", "neutral").
    pub theme: Option<String>,
    /// The background color for the Mermaid diagrams.
    pub background: Option<String>,
    /// Custom theme variables to override specific theme properties.
    pub theme_variables: Option<Value>,
}

impl Config {
    /// Extract the configuration from the `PreprocessorContext`.
    pub fn from_context(ctx: &PreprocessorContext) -> Self {
        ctx.config
            .preprocessors()
            .ok()
            .and_then(|mut map: BTreeMap<String, Config>| map.remove("mermaid-mmdr"))
            .unwrap_or_default()
    }

    #[cfg(test)]
    fn from_config(md_config: &MdBookConfig) -> Self {
        md_config
            .preprocessors()
            .ok()
            .and_then(|mut map: BTreeMap<String, Config>| map.remove("mermaid-mmdr"))
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mdbook_core::config::Config as MdBookConfig;

    #[test]
    fn test_config_empty() {
        let config = Config::from_config(&MdBookConfig::default());
        assert_eq!(config, Config::default());
    }

    #[test]
    fn test_config_with_theme() {
        let mut mdbook_config = MdBookConfig::default();
        mdbook_config
            .set("preprocessor.mermaid-mmdr.theme", "dark")
            .unwrap();

        let config = Config::from_config(&mdbook_config);
        assert_eq!(config.theme, Some("dark".to_string()));
        assert_eq!(config.background, None);
    }

    #[test]
    fn test_config_with_background() {
        let mut mdbook_config = MdBookConfig::default();
        mdbook_config
            .set("preprocessor.mermaid-mmdr.background", "transparent")
            .unwrap();

        let config = Config::from_config(&mdbook_config);
        assert_eq!(config.theme, None);
        assert_eq!(config.background, Some("transparent".to_string()));
    }

    #[test]
    fn test_config_with_both() {
        let mut mdbook_config = MdBookConfig::default();
        mdbook_config
            .set("preprocessor.mermaid-mmdr.theme", "forest")
            .unwrap();
        mdbook_config
            .set("preprocessor.mermaid-mmdr.background", "white")
            .unwrap();

        let config = Config::from_config(&mdbook_config);
        assert_eq!(config.theme, Some("forest".to_string()));
        assert_eq!(config.background, Some("white".to_string()));
    }

    #[test]
    fn test_config_with_theme_variables() {
        let mut mdbook_config = MdBookConfig::default();
        mdbook_config
            .set("preprocessor.mermaid-mmdr.theme", "modern")
            .unwrap();
        mdbook_config
            .set(
                "preprocessor.mermaid-mmdr.theme_variables.primary_color",
                "#ff0000",
            )
            .unwrap();

        let config = Config::from_config(&mdbook_config);
        assert_eq!(config.theme, Some("modern".to_string()));

        if let Some(Value::Object(map)) = config.theme_variables {
            assert_eq!(
                map.get("primary_color"),
                Some(&Value::String("#ff0000".to_string()))
            );
        } else {
            panic!("theme_variables should be an object");
        }
    }
}
