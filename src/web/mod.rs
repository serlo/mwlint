//! A REST service for mediawiki source code linting.

#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
extern crate mwlint;
extern crate mediawiki_parser;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use mediawiki_parser::*;
use mwlint::*;
use serde::{Serialize};
use serde_json::Value;

use rocket::http::Status;
use rocket::request::{Form};
use rocket::response::{self, Responder, content};
use rocket::http::Header;
use rocket::request::Request;

#[get("/")]
fn index() -> &'static str {
    "no GET endpoints available. Use POST to send mediawiki source code. \
    see https://github.com/vroland/mwlint."
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


#[derive(Debug)]
pub struct Json<T = Value>(pub T);

/// Added CORS headers to JSON contrib implementation.
impl<T: Serialize> Responder<'static> for Json<T> {
    fn respond_to(self, req: &Request) -> response::Result<'static> {
        serde_json::to_string(&self.0).map(|string| {
            {
                let mut rsp = content::Json(string).respond_to(req).unwrap();
                rsp.set_header(Header::new("Access-Control-Allow-Origin", "*"));
                rsp
            }
        }).map_err(|_e| {
            Status::InternalServerError
        })
    }
}

#[post("/", data = "<source_form>")]
fn lint(source_form: Form<SourceForm>) -> Json<ResultKind> {
    let form = source_form.get();

    let tree = match parse(&form.source) {
        Ok(elem) => elem,
        Err(mwerror) => return Json(ResultKind::Error(mwerror)),
    };

    let mut settings = Settings::default();
    settings.texvccheck_path = "".to_string();
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
        .mount("/mwlint", routes![lint, index])
        .launch();
}
