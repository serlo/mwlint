use mfnf_commons::{TemplateSpec, spec, util::CachedTexChecker};

/// Rule metadata.
#[derive(Debug, Serialize, PartialEq, Clone, Deserialize)]
pub struct RuleMeta {
    pub name: String,
    pub description: String,
}

/// Settings for linter rules.
#[derive(Serialize, Deserialize)]
pub struct Settings<'p> {
    /// Maximum allowed depth of a heading.
    pub max_heading_depth: usize,
    /// Object performing formula verification.
    #[serde(skip)]
    pub tex_checker: Option<CachedTexChecker>,
    /// Specification of allowed templates.
    #[serde(skip_deserializing)]
    pub template_spec: Vec<TemplateSpec<'p>>,
}

impl<'p> Default for Settings<'p> {
    fn default() -> Self {
        Settings {
            max_heading_depth: 4,
            tex_checker: None,
            template_spec: spec::<'p>(),
        }
    }
}

