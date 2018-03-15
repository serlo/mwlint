//! A REST service for mediawiki source code linting.

#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate mwlint;
extern crate mediawiki_parser;
#[macro_use]
extern crate serde_derive;

use mediawiki_parser::*;
use mwlint::*;

use rocket::request::{Form};
use rocket_contrib::{Json};

#[get("/")]
fn index() -> &'static str {
    "no GET endpoints available -> docs"
}

#[derive(FromForm)]
struct SourceForm {
    pub source: String,
}

#[derive(Debug, Serialize)]
enum ResultKind {
    Lints(Vec<Lint>),
    Error(MWError),
}

#[post("/", data = "<source_form>")]
fn lint(source_form: Form<SourceForm>) -> Json<ResultKind> {
    let form = source_form.get();

    let tree = match parse(&form.source) {
        Ok(elem) => elem,
        Err(mwerror) => return Json(ResultKind::Error(mwerror)),
    };

    let settings = Settings::default();
    let mut rules = get_rules();
    let mut lints = vec![];

    for mut rule in &mut rules {
        rule.run(&tree, &settings, &mut vec![])
            .expect("error while checking rule!");
        lints.append(&mut rule.lints().iter().map(|l| l.clone()).collect())
    }
    Json(ResultKind::Lints(lints))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![lint, index])
        .launch();
}
