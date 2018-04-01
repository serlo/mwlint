use template_spec::{TemplateSpec, spec};

/// Rule metadata.
#[derive(Debug, Serialize, PartialEq, Clone, Deserialize)]
pub struct RuleMeta {
    pub name: String,
    pub description: String,
}

/// Settings for linter rules.
#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Settings<'p> {
    /// Maximum allowed depth of a heading.
    pub max_heading_depth: usize,
    /// Path to the texvccheck binaries for formula checking.
    pub texvccheck_path: String,
    /// Specification of allowed templates.
    #[serde(skip_deserializing)]
    pub template_spec: Vec<TemplateSpec<'p>>,
}

impl<'p> Default for Settings<'p> {
    fn default() -> Self {
        Settings {
            max_heading_depth: 4,
            texvccheck_path: "./texvccheck".into(),
            template_spec: spec::<'p>(),
        }
    }
}

