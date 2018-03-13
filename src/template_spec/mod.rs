//! A sample template specification for "Mathe-für-Nicht-Freaks".

#[macro_use]
mod spec;
mod predicates;

use self::predicates::*;
pub use self::spec::*;
use mediawiki_parser::Element;

pub use self::spec::{TemplateSpec};

template_spec!(
    template {
        name: "formula",
        alt: ["formel", "Formel", "Formula"],
        format: Format::Inline,
        attributes: [
            attribute!(
                name: "1",
                alt: ["formel"],
                priority: Priority::Required,
                predicate: &is_math_tag
            )
        ]
    },
    template {
        name: "important",
        alt: ["-"],
        format: Format::Block,
        attributes: [
            attribute!(
                name: "1",
                alt: ["content"],
                priority: Priority::Required,
                predicate: &is_text_only_paragraph
            )
        ]
    }
);

