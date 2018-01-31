use error::*;
use utils::*;
use mediawiki_parser::*;
use settings::*;
use std::io;

mod check_template_whitelist;
mod check_headings;
mod check_lists;

macro_rules! register {
    ($list:ident, $t1:tt :: $t2:tt) => {
        $list.push(Box::new($t1::$t2::default()));
    }
}

pub fn get_rules<'e, 's>() -> Vec<Box<Rule<'e, 's>>> {
    let mut rules: Vec<Box<Rule<'e, 's>>> = vec![];
    register!(rules, check_headings::CheckHeadings);
    register!(rules, check_lists::CheckLists);
    register!(rules, check_template_whitelist::CheckTemplateWhitelist);
    rules
}
