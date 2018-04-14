use rule::*;
use lint::{Example, LintKind};

mod check_headings;
mod check_lists;
mod check_templates;
mod check_formulas;

pub use self::check_headings::CheckHeadings;
pub use self::check_lists::CheckLists;
pub use self::check_templates::CheckTemplates;
pub use self::check_formulas::CheckFormulas;

macro_rules! register {
    ($list:ident, $t1:tt :: $t2:tt) => {
        $list.push(Box::new($t1::$t2::new()));
    }
}

type RuleList<'e, 's: 'e> = Vec<Box<Rule<'e, 's>>>;

/// Get a list of all available rules.
pub fn get_rules<'e, 's: 'e>() -> RuleList<'e, 's> {
    let mut rules: RuleList<'e, 's> = vec![];
    register!(rules, check_headings::CheckHeadings);
    register!(rules, check_lists::CheckLists);
    register!(rules, check_templates::CheckTemplates);
    register!(rules, check_formulas::CheckFormulas);
    rules
}

/// Find all examples for lints of a given kind.
pub fn get_examples<'e, 'r, 's: 'e>(
    rules: &'r RuleList<'e, 's>,
    kind: LintKind
) -> Vec<&'r Example> {

    let mut result = vec![];
    for rule in rules {
        result.append(&mut rule.examples().iter().filter(|e| e.kind == kind).collect());
    }
    result
}
