use mediawiki_parser::*;
use std::io;

/// Specifies wether a template represents a logical unit (`Block`)
/// or simpler markup (`Inline`).
pub enum Format {
    Block,
    Inline
}

/// Template attributes can have different priorities.
pub enum Priority {
    Required,
    Optional
}

/// A function to determine wether a given element is allowed.
type Predicate = Fn(&Element) -> bool;

/// Represents a (semantic) template.
pub struct TemplateSpec<'p> {
    pub name: String,
    pub alternative_names: Vec<String>,
    pub format: Format,
    pub attributes: Vec<Attribute<'p>>,
}

/// Represents an attribute (or argument) of a template.
pub struct Attribute<'p> {
    pub name: String,
    pub alternative_names: Vec<String>,
    pub priority: Priority,
    pub predicate: &'p Predicate,
    pub predicate_source: String,
}

/// Checks a predicate for a given input tree.
#[derive(Default)]
struct TreeChecker<'path> {
    pub path: Vec<&'path Element>,
    pub result: bool,
}

#[derive(Clone, Copy)]
enum CheckerMode {
    All,
    None,
}

struct CheckerSettings<'p> {
    pub predicate: &'p Predicate,
    pub mode: CheckerMode,
}

impl <'e, 'p: 'e> Traversion<'e, &'p CheckerSettings<'p>> for TreeChecker<'e> {

    path_methods!('e);

    fn work(
        &mut self,
        root: &Element,
        settings: &'p CheckerSettings<'p>,
        _: &mut io::Write
    ) -> io::Result<bool> {
        match settings.mode {
            CheckerMode::All => self.result &= (settings.predicate)(root),
            CheckerMode::None => self.result &= !(settings.predicate)(root),
        }
        Ok(true)
    }
}

impl<'p> TreeChecker<'p> {
    pub fn all(root: &Element, predicate: &Predicate) -> bool {
        let settings = CheckerSettings {
            predicate,
            mode: CheckerMode::All
        };
        let mut checker = TreeChecker::default();
        checker.result = true;
        checker.run(&root, &settings, &mut vec![])
            .expect("error checking predicate!");
        checker.result
    }

    pub fn min_one(root: &Element, predicate: &Predicate) -> bool {
        !TreeChecker::never(root, predicate)
    }

    pub fn never(root: &Element, predicate: &Predicate) -> bool {
        let settings = CheckerSettings {
            predicate,
            mode: CheckerMode::None
        };
        let mut checker = TreeChecker::default();
        checker.result = true;
        checker.run(&root, &settings, &mut vec![])
            .expect("error checking predicate!");
        checker.result
    }
}

pub fn true_for_subtree(root: &Element, predicate: &Predicate) -> bool {
    true
}

macro_rules! template_spec {
    ($($template:expr),*) => {
        fn specs<'p>() -> Vec<TemplateSpec<'p>> {
            vec![$($template),*]
        }
    }
}

macro_rules! template {
    (
        name: $name:expr,
        alt: [$($altname:expr),*],
        format: $format:expr,
        attributes: [$($attr:expr),*]
    ) => {
        TemplateSpec {
            name: $name.into(),
            alternative_names: vec![$($altname.into()),*],
            format: $format,
            attributes: vec![$($attr),*]
        }
    }
}

macro_rules! attribute {
    (
        name: $name:expr,
        alt: [$($altname:expr),*],
        priority: $priority:expr,
        predicate: $predicate:expr
    ) => {
        Attribute {
            name: $name.into(),
            alternative_names: vec![$($altname.into()),*],
            priority: $priority,
            predicate: $predicate,
            predicate_source: stringify!($predicate).into()
        }
    }
}
