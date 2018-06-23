#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate mwlint;
extern crate mediawiki_parser;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;
use mediawiki_parser::{MWError};

#[derive(Debug, Serialize)]
enum LintResult {
    Lints(Vec<mwlint::Lint>),
    Error(MWError),
}

/// Naive linter function. Outputs result as serialized JSON.
#[wasm_bindgen]
pub fn lint(input: &str) -> String {
    let settings = mwlint::Settings::default();

    let mut tree = mediawiki_parser::parse(&input)
        .map_err(|e| LintResult::Error(e));

    tree = tree.and_then(|t|
            mwlint::normalize(t, &settings)
                .map_err(|e| LintResult::Error(MWError::TransformationError(e))));

    let result = tree.map(|tree| {
        let mut rules = mwlint::get_rules();
        let mut lints = vec![];

        for mut rule in &mut rules {
            rule.run(&tree, &settings, &mut vec![])
                .expect("error while checking rule!");
            lints.append(&mut rule.lints().iter().map(|l| l.clone()).collect())
        }
        LintResult::Lints(lints)
    });

    serde_json::to_string(&result)
        .expect("could not serialize lints")
}

/// Lint examples as JSON string.
#[wasm_bindgen]
pub fn examples() -> String {
    let rules = mwlint::get_rules();
    let examples = rules.iter().fold(vec![], |mut vec, rule| {
        vec.append(&mut rule.examples().clone());
        vec
    });

    serde_json::to_string(&examples)
        .expect("could not serialize examples")
}

