use utils::*;

/// The general settings structure.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Settings<'a> {
    pub parameters: RuleParameters,
    pub rules: Vec<Rule<'a>>,
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

impl<'a> Settings<'a> {
    pub fn append_rules(&mut self, rules: &mut Vec<Rule<'a>>) {
        for rule in rules.drain(..) {
            let mut found = false;
            for existing in &self.rules {
                if existing.name == rule.name {
                    found = true;
                    break;
                }
            }
            if !found {
                self.rules.push(rule);
            }
        }
    }
}
