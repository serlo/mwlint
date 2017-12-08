
/// The general settings structure.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    pub parameters: RuleParameters,
}

/// Parameters for linter rules.
#[derive(Debug, Serialize, Deserialize)]
pub struct RuleParameters {
    pub max_heading_depth: usize,
}

impl Default for RuleParameters {
    fn default() -> Self {
        RuleParameters { max_heading_depth: 4 }
    }
}

