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
    },
    template {
        name: "definition",
        alt: [":Mathe für Nicht-Freaks: Vorlage:Definition"],
        format: Format::Block,
        attributes: [
            attribute!(
                name: "title",
                alt: ["titel"],
                priority: Priority::Required,
                predicate: &is_text_only_paragraph
            ),
            attribute!(
                name: "definition",
                alt: [],
                priority: Priority::Required,
                predicate: &is_text_only_paragraph
            )
        ]
    },
    template {
        name: "theorem",
        alt: [":Mathe für Nicht-Freaks: Vorlage:Satz"],
        format: Format::Block,
        attributes: [
            attribute!(
                name: "title",
                alt: ["titel"],
                priority: Priority::Required,
                predicate: &is_text_only_paragraph
            ),
            attribute!(
                name: "theorem",
                alt: ["satz"],
                priority: Priority::Required,
                predicate: &is_text_only_paragraph
            )
        ]
    },
    template {
        name: "example",
        alt: [":Mathe für Nicht-Freaks: Vorlage:Beispiel"],
        format: Format::Block,
        attributes: [
            attribute!(
                name: "title",
                alt: ["titel"],
                priority: Priority::Required,
                predicate: &is_text_only_paragraph
            ),
            attribute!(
                name: "example",
                alt: ["beispiel"],
                priority: Priority::Required,
                predicate: &is_text_only_paragraph
            )
        ]
    }
);

