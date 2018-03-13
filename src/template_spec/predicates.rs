//! Some samle predicates for "Mathe-für-Nicht-Freaks".

use mediawiki_parser::*;

pub fn is_math_tag(elem: &Element) -> bool {
    if let Element::Formatted { ref markup, .. } = *elem {
        *markup == MarkupType::Math
    } else {
        false
    }
}
