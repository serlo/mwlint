use lint::{Example, LintKind};
use rule::*;

mod check_formulas;
mod check_headings;
mod check_html;
mod check_lists;
mod check_templates;

pub use self::check_formulas::CheckFormulas;
pub use self::check_headings::CheckHeadings;
pub use self::check_html::CheckHtml;
pub use self::check_lists::CheckLists;
pub use self::check_templates::CheckTemplates;

macro_rules! register {
    ($list:ident, $t1:tt :: $t2:tt) => {
        $list.push(Box::new($t1::$t2::default()));
    };
}

/// Get a list of all available rules.
pub fn get_rules<'e, 's: 'e>() -> Vec<Box<Rule<'e, 's>>> {
    let mut rules: Vec<Box<Rule<'e, 's>>> = vec![];
    register!(rules, check_headings::CheckHeadings);
    register!(rules, check_lists::CheckLists);
    register!(rules, check_templates::CheckTemplates);
    register!(rules, check_formulas::CheckFormulas);
    register!(rules, check_html::CheckHtml);
    rules
}

/// Find all examples for lints of a given kind.
pub fn get_examples<'e, 'r, 's: 'e>(
    rules: &'r [Box<Rule<'e, 's>>],
    kind: LintKind,
) -> Vec<&'r Example> {
    let mut result = vec![];
    for rule in rules {
        result.append(&mut rule.examples().iter().filter(|e| e.kind == kind).collect());
    }
    result
}
