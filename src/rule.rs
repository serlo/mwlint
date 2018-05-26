use mediawiki_parser::*;
use settings::{Settings, RuleMeta};
use lint::*;
use std::io;


/// Linter rule trait.
pub trait Rule<'e, 's: 'e>: Traversion<'e, &'s Settings<'s>> + 'e {
    fn meta(&self) -> RuleMeta;
    fn push(&mut self, lint: Lint);
    fn lints(&self) -> &Vec<Lint>;
    fn examples(&self) -> &Vec<Example>;
}

macro_rules! rule_impl {
    ($t:ident, $desc:expr =>
        examples: $(
            $name:ident,
            $bad:expr,
            $bad_expl:expr,
            $good:expr,
            $good_expl:expr
            => $result:expr
        );*
    ) => {
        #[doc = $desc]
        ///# Examples
        $(
            /// # example
            #[doc = $bad_expl]
            ///
            ///```text
            #[doc = $bad]
            ///```
            ///
            #[doc = $good_expl]
            ///
            ///```text
            #[doc = $good]
            ///```
        )*

        pub struct $t<'e> {
            pub path: Vec<&'e Element>,
            pub lints: Vec<Lint>,
            pub examples: Vec<Example>,
        }

        impl<'e> Default for $t<'e> {
            fn default() -> $t<'e> {
                $t {
                    examples: vec![
                        $(
                        Example {
                            name: stringify!($name).into(),
                            bad: $bad.into(),
                            bad_explanation: $bad_expl.into(),
                            good: $good.into(),
                            good_explanation: $good_expl.into(),
                            kind: $result.into(),
                        },
                        )*
                    ],
                    path: vec![],
                    lints: vec![],
                }
            }
        }
        $(
            #[test]
            fn $name() {
                let bad_input = $bad;
                let good_input = $good;
                let tree_bad = parse(bad_input).unwrap();
                let tree_good = parse(good_input).unwrap();
                let mut settings = Settings::default();
                settings.tex_checker = Some(
                    CachedTexChecker::new(&PathBuf::from("./texvccheck"), 10)
                );
                let mut rule_bad = $t::default();
                let bad_lints = tree_bad.check(&mut rule_bad, &settings)
                    .expect("rule crashed!");
                let mut rule_good = $t::default();
                let good_lints = tree_good.check(&mut rule_good, &settings)
                    .expect("rule crashed!");

                let mut bad_matches = 0;
                for lint in bad_lints {
                    if $result == lint.kind {
                        bad_matches += 1;
                    }
                }

                assert!(bad_matches >= 1);
                eprintln!("{:#?}", good_lints);
                assert_eq!(good_lints.len(), 0);
            }
        )*

        impl<'e, 's: 'e> Rule<'e, 's> for $t<'e> {
            fn meta(&self) -> RuleMeta {
                RuleMeta {
                    name: stringify!($t).into(),
                    description: $desc.into(),
                }
            }

            fn push(&mut self, lint: Lint) {
                self.lints.push(lint);
            }

            fn lints(&self) -> &Vec<Lint> {
                &self.lints
            }

            fn examples(&self) -> &Vec<Example>{
                &self.examples
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
    ) -> io::Result<&Vec<Lint>>;
}

impl Checkable for Element {
    fn check<'e, 's>(
        &'e self,
        rule: &mut Rule<'e, 's>,
        settings: &'s Settings,
    ) -> io::Result<&Vec<Lint>> {

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
