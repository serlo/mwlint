use mediawiki_parser::*;
use std::io;
use std::fmt;

/// Specifies wether a template represents a logical unit (`Block`)
/// or simpler markup (`Inline`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Format {
    Block,
    Inline
}

/// Template attributes can have different priorities.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    Required,
    Optional
}

/// A function to determine wether a given element is allowed.
type Predicate = Fn(&[Element]) -> bool;

/// Represents a (semantic) template.
#[derive(Debug, Clone, Serialize)]
pub struct TemplateSpec<'p> {
    pub name: String,
    pub alternative_names: Vec<String>,
    pub format: Format,
    pub attributes: Vec<Attribute<'p>>,
}

/// Represents an attribute (or argument) of a template.
#[derive(Clone, Serialize)]
pub struct Attribute<'p> {
    pub name: String,
    pub alternative_names: Vec<String>,
    pub priority: Priority,
    #[serde(skip)]
    pub predicate: &'p Predicate,
    pub predicate_source: String,
}

impl<'p> fmt::Debug for Attribute<'p> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Attribute: {{ name: {:?}, alternative_names {:?}, \
                   priority: {:?}, predicate: <predicate func>, \
                   predicate_source: {:?} }}", self.name, self.alternative_names,
                   self.priority, self.predicate_source)
    }
}

/// Checks a predicate for a given input tree.
#[derive(Default)]
pub struct TreeChecker<'path> {
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

    fn work_vec(
        &mut self,
        root: &[Element],
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
    pub fn all(root: &[Element], predicate: &Predicate) -> bool {
        let settings = CheckerSettings {
            predicate,
            mode: CheckerMode::All
        };
        let mut checker = TreeChecker::default();
        checker.result = true;
        checker.run_vec(&root, &settings, &mut vec![])
            .expect("error checking predicate!");
        checker.result
    }

    pub fn min_one(root: &[Element], predicate: &Predicate) -> bool {
        !TreeChecker::never(root, predicate)
    }

    pub fn never(root: &[Element], predicate: &Predicate) -> bool {
        let settings = CheckerSettings {
            predicate,
            mode: CheckerMode::None
        };
        let mut checker = TreeChecker::default();
        checker.result = true;
        checker.run_vec(&root, &settings, &mut vec![])
            .expect("error checking predicate!");
        checker.result
    }
}

pub fn check_name(name: &[Element]) -> Option<&str> {
    if name.len() != 1 {
        return None
    }
    match name.first() {
        Some(&Element::Text { ref text, .. }) => return Some(text),
        Some(&Element::Paragraph { ref content, .. }) => {
            if content.len() != 1 {
                return None
            }
            if let Some(&Element::Text { ref text, .. }) = content.first() {
                return Some(text)
            }
        },
        _ => (),
    };
    None
}

macro_rules! template_spec {
    ($(
        template {
            name: $name:expr,
            alt: [$($altname:expr),*],
            format: $format:expr,
            attributes: [$($attr:expr),*]
        }
    ),*) => {
        pub fn spec<'p>() -> Vec<TemplateSpec<'p>> {
            vec![
                $(
                    TemplateSpec {
                        name: $name.into(),
                        alternative_names: vec![$($altname.into()),*],
                        format: $format,
                        attributes: vec![$($attr),*]
                    }
                ),*
            ]
        }

        pub fn format(template: &Element) -> Option<Format> {
            if let Element::Template { ref name, .. } = *template {
                match check_name(name) {
                    $(
                        Some($name) => Some($format),
                        $(Some($altname) => Some($format)),*
                    ),*,
                    _ => None
                }
            } else {
                None
            }
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
