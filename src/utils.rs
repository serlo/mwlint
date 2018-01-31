use error;
use mediawiki_parser::*;
use settings::*;
use std::fmt;

/// Linter rule trait.
pub trait Rule<'e, 's>: Traversion<'e, &'s RuleParameters> + 'e {
    fn meta(&self) -> RuleMeta;
    fn push(&mut self, lint: error::Lint);
    fn lints(&self) -> &Vec<error::Lint>;
}

macro_rules! rule_impl {
    ($name:expr, $desc:expr, $t:tt) => {
        impl<'e, 's> Rule<'e, 's> for $t<'e> {
            fn meta(&self) -> RuleMeta {
                RuleMeta {
                    name: $name.into(),
                    description: $desc.into(),
                }
            }

            fn push(&mut self, lint: ::error::Lint) {
                self.lints.push(lint);
            }

            fn lints(&self) -> &Vec<::error::Lint> {
                &self.lints
            }
        }
    }
}

macro_rules! path_impl {
    () => {
        fn path_push(&mut self, e: &'e Element) { self.path.push(e) }
        fn path_pop(&mut self) -> Option<&'e Element> { self.path.pop() }
        fn get_path(&self) -> &Vec<&'e Element> { &self.path }
    }
}
