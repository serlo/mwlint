use utils::*;

/// Rule metadata.
#[derive(Debug, Serialize, PartialEq, Clone, Deserialize)]
pub struct RuleMeta {
    pub name: String,
    pub description: String,
}

/// Settings for linter rules.
#[derive(Debug, Serialize, PartialEq, Clone, Deserialize)]
pub struct Settings {
    /// Maximum allowed depth of a heading.
    pub max_heading_depth: usize,
    /// A list of allowed template names. If empty, all templates are allowed.
    pub template_whitelist: Vec<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            max_heading_depth: 4,
            template_whitelist: vec![],
        }
    }
}

