use error;
use mediawiki_parser::ast::*;

/// Signature of a linter function.
type LintFunc = Fn(&Element, &mut Vec<&Element>, &mut Vec<error::Lint>);

/// Function appying a linter function to a list of elements.
type LintVec = Fn(&LintFunc, &Vec<Element>, &mut Vec<&Element>, &mut Vec<error::Lint>);

/// Execute a lint function for every element of a list.
pub fn lint_vec(func: &LintFunc, elements: &Vec<Element>, path: &mut Vec<&Element>, lints: &mut Vec<error::Lint>) {

    for element in &elements[..] {
        let mut sublints = vec![];
        func(element, path, &mut sublints);
        lints.append(&mut sublints);
    }
}

/// Execute a lint function for a tree element recursively.
pub fn lint_elem<'a>(elem_func: &LintFunc, root: &'a Element, path: &mut Vec<&'a Element>, lints: &mut Vec<error::Lint>) {

    lint_elem_template(elem_func, &lint_vec, root, path, lints);
}


/// A helper function for traversing a syntax tree recursively.
pub fn lint_elem_template<'a>(elem_func: &LintFunc, vec_func: &LintVec, root: &'a Element, path: &mut Vec<&'a Element>, lints: &mut Vec<error::Lint>) {

    let mut sublints = vec![];
    path.push(root);
    match root {
        &Element::Document { ref content, .. } => {
            vec_func(elem_func, content, path, &mut sublints);
        },
        &Element::Heading { ref caption, ref content, .. } => {
            vec_func(elem_func, caption, path, &mut sublints);
            vec_func(elem_func, content, path, &mut sublints);
        },
        &Element::Text { .. } => (),
        &Element::Formatted { ref content, .. } => {
            vec_func(elem_func, content, path, &mut sublints);
        },
        &Element::Paragraph { ref content, .. } => {
            vec_func(elem_func, content, path, &mut sublints);
        },
        &Element::Template { ref content, .. } => {
            vec_func(elem_func, content, path, &mut sublints);
        },
        &Element::TemplateArgument { ref value, .. } => {
            vec_func(elem_func, value, path, &mut sublints);
        },
        &Element::InternalReference { ref target, ref options, ref caption, .. } => {
            vec_func(elem_func, target, path, &mut sublints);
            for option in options {
                vec_func(elem_func, option, path, &mut sublints);
            }
            vec_func(elem_func, caption, path, &mut sublints);
        },
        &Element::ExternalReference { ref caption, .. } => {
            vec_func(elem_func, caption, path, &mut sublints);
        },
        &Element::ListItem { ref content, .. } => {
            vec_func(elem_func, content, path, &mut sublints);
        },
        &Element::List { ref content, .. } => {
            vec_func(elem_func, content, path, &mut sublints);
        },
        &Element::Table {ref caption, ref rows,  .. } => {
            vec_func(elem_func, caption, path, &mut sublints);
            vec_func(elem_func, rows, path, &mut sublints);
        },
        &Element::TableRow { ref cells, .. } => {
            vec_func(elem_func, cells, path, &mut sublints);
        },
        &Element::TableCell { ref content, .. } => {
            vec_func(elem_func, content, path, &mut sublints);
        },
        &Element::Comment { .. } => (),
        &Element::HtmlTag { ref content, .. } => {
            vec_func(elem_func, content, path, &mut sublints);
        }
    }
    path.pop();
    lints.append(&mut sublints);
}
