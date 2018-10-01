extern crate wasm_bindgen;
extern crate mwlint;
extern crate mediawiki_parser;
extern crate serde_json;
extern crate pulldown_cmark;

#[macro_use]
extern crate serde_derive;

use wasm_bindgen::prelude::*;
use mediawiki_parser::{MWError};
use pulldown_cmark::{html, Parser};

#[derive(Debug, Serialize)]
enum LintResult {
    Lints(Vec<mwlint::Lint>),
    Error(MWError),
}

fn render_string(input: &mut String) {
    let clone = input.clone();
    let parser = Parser::new(&clone);
    input.clear();
    html::push_html(input, parser);
}

fn render_lint(lint: &mut mwlint::Lint) {
    render_string(&mut lint.explanation);
    render_string(&mut lint.explanation_long);
    render_string(&mut lint.solution);
}

fn render_example(example: &mut mwlint::Example) {
    render_string(&mut example.bad_explanation);
    render_string(&mut example.good_explanation);
    example.good = format!("<pre><code>{}</code></pre>", &example.good);
    example.bad = format!("<pre><code>{}</code></pre>", &example.bad);
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

        for mut lint in &mut lints {
            render_lint(&mut lint);
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
    let examples = rules
        .iter()
        .fold(vec![], |mut vec, rule| {
            let mut rule_examples = rule.examples()
                .iter()
                .map(|example| {
                    let mut e = example.clone();
                    render_example(&mut e);
                    e
                })
                .collect();
            vec.append(&mut rule_examples);
            vec
        });

    serde_json::to_string(&examples)
        .expect("could not serialize examples")
}

