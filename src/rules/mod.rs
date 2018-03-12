use rule::*;
use lint::*;

//mod check_template_whitelist;
mod check_headings;
mod check_lists;

pub use self::check_headings::*;
pub use self::check_lists::*;

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
    //register!(rules, check_template_whitelist::CheckTemplateWhitelist);
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
