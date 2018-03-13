//! A sample template specification for "Mathe-für-Nicht-Freaks".

use predicates::*;
use template_spec::*;

template_spec!(
    template!(
        name: "formula",
        alt: ["formel"],
        format: Format::Inline,
        attributes: [
            attribute!(
                name: "1",
                alt: [],
                priority: Priority::Required,
                predicate: &is_math_tag
            )
        ]
    )
);

