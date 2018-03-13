//! Some samle predicates for "Mathe-für-Nicht-Freaks".

use mediawiki_parser::*;

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
