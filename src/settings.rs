use utils::*;

/*
/// The general settings structure.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Settings<'a> {
    pub parameters: RuleParameters,
    pub rules: Vec<Rule<'a>>,
}
*/

/// Rule metadata.
#[derive(Debug, Serialize, PartialEq, Clone, Deserialize)]
pub struct RuleMeta {
    pub name: String,
    pub description: String,
}

/// Parameters for linter rules.
#[derive(Debug, Serialize, PartialEq, Clone, Deserialize)]
pub struct RuleParameters {
    /// Maximum allowed depth of a heading.
    pub max_heading_depth: usize,
    /// A list of allowed template names. If empty, all templates are allowed.
    pub template_whitelist: Vec<String>,
}

impl Default for RuleParameters {
    fn default() -> Self {
        RuleParameters {
            max_heading_depth: 4,
            template_whitelist: vec![],
        }
    }
}

/*
impl<'a> Settings<'a> {
    /// merge a list of rules into this settings struct.
    pub fn merge_rules(&mut self, rules: &mut Vec<Rule<'a>>) {
        'others: for rule in rules.drain(..) {
            for existing in &mut self.rules {
                if existing.name == rule.name {
                    existing.description = rule.description;
                    existing.function = rule.function;
                    continue 'others;
                }
            }
            self.rules.push(rule);
        }
    }
}
*/
