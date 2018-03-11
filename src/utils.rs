use error;
use mediawiki_parser::*;
use settings::*;
use error::*;
use std::fmt;
use std::io;

/// Linter rule trait.
pub trait Rule<'e, 's>: Traversion<'e, &'s Settings> + 'e {
    fn meta(&self) -> RuleMeta;
    fn push(&mut self, lint: LintType);
    fn lints(&self) -> &Vec<LintType>;
    fn examples(&self) -> Vec<Example>;
}

macro_rules! rule_impl {
    ($t:ident, $desc:expr =>
        examples: $(
            $name:ident,
            $bad:expr,
            $bad_expl:expr,
            $good:expr,
            $good_expl:expr
            => $result:pat
        )|*
    ) => {
        #[doc = $desc]
        ///# Examples
        $(
            ///## Good:
            ///
            ///```text
            #[doc = $good]
            ///```
            ///
            #[doc = $good_expl]
            ///
            ///## Bad
            ///
            ///```text
            #[doc = $bad]
            ///```
            #[doc = $bad_expl]
            ///
        )*
        #[derive(Debug, Default)]
        pub struct $t<'e> {
            pub path: Vec<&'e Element>,
            pub lints: Vec<LintType>,
        }
        $(
            #[test]
            fn $name() {
                let bad_input = $bad;
                let good_input = $good;
                let tree_bad = parse(bad_input).unwrap();
                let tree_good = parse(good_input).unwrap();
                let settings = Settings::default();
                let mut rule_bad = $t::default();
                let bad_lints = tree_bad.check(&mut rule_bad, &settings)
                    .expect("rule crashed!");
                let mut rule_good = $t::default();
                let good_lints = tree_good.check(&mut rule_good, &settings)
                    .expect("rule crashed!");

                let mut bad_matches = 0;
                for lint in bad_lints {
                    if let $result = *lint {
                        bad_matches += 1;
                    }
                }

                assert!(bad_matches >= 1);
                assert_eq!(good_lints.len(), 0);
            }
        )*

        impl<'e, 's> Rule<'e, 's> for $t<'e> {
            fn meta(&self) -> RuleMeta {
                RuleMeta {
                    name: stringify!($t).into(),
                    description: $desc.into(),
                }
            }

            fn push(&mut self, lint: LintType) {
                self.lints.push(lint);
            }

            fn lints(&self) -> &Vec<LintType> {
                &self.lints
            }

            fn examples(&self) -> Vec<Example>{
                vec![]
            }

        }
    }
}

/// This object can be rendered by a traversion.
pub trait Checkable  {
    fn check<'e, 's>(
        &'e self,
        rule: &mut Rule<'e, 's>,
        settings: &'s Settings,
    ) -> io::Result<&Vec<LintType>>;
}

impl Checkable for Element {
    fn check<'e, 's>(
        &'e self,
        rule: &mut Rule<'e, 's>,
        settings: &'s Settings,
    ) -> io::Result<&Vec<LintType>> {

        rule.run(self, settings, &mut vec![])?;
        Ok(rule.lints())
    }
}

macro_rules! path_impl {
    () => {
        fn path_push(&mut self, e: &'e Element) { self.path.push(e) }
        fn path_pop(&mut self) -> Option<&'e Element> { self.path.pop() }
        fn get_path(&self) -> &Vec<&'e Element> { &self.path }
    }
}
