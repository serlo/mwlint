//! Some samle predicates for "Mathe-für-Nicht-Freaks".

use mediawiki_parser::*;
use super::{Format, TreeChecker};
use super::format;

pub fn is_math_tag(elems: &[Element]) -> bool {
    if elems.len() != 1 {
        return false
    }
    if let Some(&Element::Formatted { ref markup, .. }) = elems.first() {
        *markup == MarkupType::Math
    } else {
        false
    }
}

pub fn is_text_only_paragraph(elems: &[Element]) -> bool {
    fn shallow(elements: &[Element]) -> bool {
        for elem in elements {
            match *elem {
                Element::Template { .. } => {
                    if format(elem) != Some(Format::Inline) {
                        return false
                    }
                },
                Element::Gallery { .. }
                | Element::Heading { .. }
                | Element::Table { .. }
                | Element::TableRow { .. }
                | Element::TableCell { .. }
                | Element::InternalReference { .. }
                | Element::ListItem { .. }
                => return false,
                _ => (),
            }
        }
        true
    };
    TreeChecker::all(elems, &shallow)
}
