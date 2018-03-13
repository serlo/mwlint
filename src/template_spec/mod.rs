//! A sample template specification for "Mathe-für-Nicht-Freaks".

mod predicates;
#[macro_use]
mod spec;

use self::predicates::*;
pub use self::spec::*;

pub use self::spec::{TemplateSpec};

template_spec!(
    template!(
        name: "formula",
        alt: ["formel"],
        format: Format::Inline,
        attributes: [
            attribute!(
                name: "1",
                alt: ["formel"],
                priority: Priority::Required,
                predicate: &is_math_tag
            )
        ]
    )
);

