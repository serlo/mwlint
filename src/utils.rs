use error;
use mediawiki_parser::ast::*;
use settings::Settings;
use std::fmt;

/// Signature of a linter function.
pub type LintFunc<'a> = Fn(&'a Element,
                       &mut Vec<&'a Element>,
                       &Settings,
                       &mut Vec<error::Lint>);

/// Function appying a linter function to a list of elements.
pub type LintVec<'a> = Fn(&LintFunc<'a>,
                      &'a Vec<Element>,
                      &mut Vec<&'a Element>,
                      &Settings,
                      &mut Vec<error::Lint>);

/// Execute a lint function for every element of a list.
pub fn lint_vec<'a>(
    func: &LintFunc<'a>,
    elements: &'a Vec<Element>,
    path: &mut Vec<&'a Element>,
    settings: &Settings,
    lints: &mut Vec<error::Lint>,
) {

    for element in elements {
        let mut sublints = vec![];
        func(&element, path, settings, &mut sublints);
        lints.append(&mut sublints);
    }
}

/// Execute a lint function for a tree element recursively.
pub fn lint_elem<'a>(
    elem_func: &LintFunc<'a>,
    root: &'a Element,
    path: &mut Vec<&'a Element>,
    settings: &Settings,
    lints: &mut Vec<error::Lint>,
) {

    lint_elem_template(elem_func, &lint_vec, root, path, settings, lints);
}

/// Metadata of a linter rule.
#[derive(Serialize, Deserialize)]
pub struct Rule<'a> {
    pub description: String,
    pub name: String,
    pub enable: bool,

    #[serde(skip)]
    pub function: Option<&'a LintFunc<'a>>,
}

/// Reads all enclosed functions and builds a function returning rule meta data.
///
/// Signature of the new function: `pub fn get_rules() -> Vec<Rule>`
macro_rules! build_rule_meta {
    ($(#[doc=$doc:expr] pub fn $name:ident <$lt:tt> ($($rest:tt)*) $body:block)*) => {
        /// Collect metadata of enclosed rules and return it at run time.
        pub fn get_rules<'a>() -> Vec<Rule<'a>> {
            let mut rules = vec![];
                $(
                    rules.push(Rule {
                        description: $doc.trim().to_string(),
                        name: stringify!($name).to_string(),
                        enable: true,
                        function: Some(&$name),
                    });
                )*
            rules
        }
        $(
            #[doc=$doc]
            pub fn $name <$lt> ($( $rest )*) $body
        )*
    };
    ($($t:tt)*) => {
        compile_error!(
"A linter rule must have the following signature:

/// mandatory description
pub fn func_name<'root_lifetime>(<args>) {..}

If you want to define helper structures or functions,
define them inside your function or outside of build_rule_meta!.");
    }
}


/// A helper function for traversing a syntax tree recursively.
pub fn lint_elem_template<'a>(
    elem_func: &LintFunc<'a>,
    vec_func: &LintVec<'a>,
    root: &'a Element,
    path: &mut Vec<&'a Element>,
    settings: &Settings,
    lints: &mut Vec<error::Lint>,
) {

    let mut sublints = vec![];
    path.push(root);
    match root {
        &Element::Document { ref content, .. } => {
            vec_func(elem_func, content, path, settings, &mut sublints);
        }
        &Element::Heading {
            ref caption,
            ref content,
            ..
        } => {
            vec_func(elem_func, caption, path, settings, &mut sublints);
            vec_func(elem_func, content, path, settings, &mut sublints);
        }
        &Element::Text { .. } => (),
        &Element::Formatted { ref content, .. } => {
            vec_func(elem_func, content, path, settings, &mut sublints);
        }
        &Element::Paragraph { ref content, .. } => {
            vec_func(elem_func, content, path, settings, &mut sublints);
        }
        &Element::Template { ref content, .. } => {
            vec_func(elem_func, content, path, settings, &mut sublints);
        }
        &Element::TemplateArgument { ref value, .. } => {
            vec_func(elem_func, value, path, settings, &mut sublints);
        }
        &Element::InternalReference {
            ref target,
            ref options,
            ref caption,
            ..
        } => {
            vec_func(elem_func, target, path, settings, &mut sublints);
            for option in options {
                vec_func(elem_func, option, path, settings, &mut sublints);
            }
            vec_func(elem_func, caption, path, settings, &mut sublints);
        }
        &Element::ExternalReference { ref caption, .. } => {
            vec_func(elem_func, caption, path, settings, &mut sublints);
        }
        &Element::ListItem { ref content, .. } => {
            vec_func(elem_func, content, path, settings, &mut sublints);
        }
        &Element::List { ref content, .. } => {
            vec_func(elem_func, content, path, settings, &mut sublints);
        }
        &Element::Table {
            ref caption,
            ref rows,
            ..
        } => {
            vec_func(elem_func, caption, path, settings, &mut sublints);
            vec_func(elem_func, rows, path, settings, &mut sublints);
        }
        &Element::TableRow { ref cells, .. } => {
            vec_func(elem_func, cells, path, settings, &mut sublints);
        }
        &Element::TableCell { ref content, .. } => {
            vec_func(elem_func, content, path, settings, &mut sublints);
        }
        &Element::Comment { .. } => (),
        &Element::HtmlTag { ref content, .. } => {
            vec_func(elem_func, content, path, settings, &mut sublints);
        },
        &Element::Error { .. } => (),
    }
    path.pop();
    lints.append(&mut sublints);
}

impl<'a> fmt::Debug for Rule<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rule {{ name: {}, enable: {} }}", &self.name, self.enable)
    }
}

