use mfnf_template_spec::{spec, spec_meta::TemplateSpec};
use mwparser_utils::CachedTexChecker;

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
    /// List of allowed html tags.
    pub html_whitelist: Vec<String>,
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
            html_whitelist: vec!["section".into(), "dfn".into(), "ref".into()],
            tex_checker: None,
            template_spec: spec::<'p>(),
        }
    }
}
