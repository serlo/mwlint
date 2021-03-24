extern crate mediawiki_parser;
extern crate serde_json;
extern crate serde_yaml;
#[macro_use]
extern crate structopt;
extern crate mfnf_template_spec;
extern crate mwlint;
extern crate mwparser_utils;

use mfnf_template_spec::markdown;
use mwlint::*;
use mwparser_utils::CachedTexChecker;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "mwlint",
    about = "Takes the syntax tree of a mediawiki document \
             (as created by `mwtoast`) as input and checks it for for discouraged \
             patterns and other nitpicks."
)]
struct Args {
    /// Dump the default settings to stdout.
    #[structopt(long = "dump-config")]
    dump_config: bool,
    /// Dump the template specification as markdown.
    #[structopt(long = "dump-docs")]
    dump_template_docs: bool,
    /// Path to the input file.
    #[structopt(parse(from_os_str), short = "i", long = "input")]
    input_file: Option<PathBuf>,
    /// Path to the config file.
    #[structopt(parse(from_os_str), short = "c", long = "config")]
    config: Option<PathBuf>,
    /// Path to the texvccheck binary (formula checking).
    #[structopt(parse(from_os_str), short = "p", long = "texvccheck-path")]
    texvccheck_path: Option<PathBuf>,
}

fn main() -> Result<(), std::io::Error> {
    let args = Args::from_args();

    let mut settings = if let Some(path) = args.config {
        let file = fs::File::open(&path)?;
        serde_yaml::from_reader(&file).expect("Error reading settings:")
    } else {
        Settings::default()
    };

    // dump settings
    if args.dump_config {
        println!(
            "{}",
            serde_yaml::to_string(&settings).expect("Could serialize settings!")
        );
        process::exit(0);
    }

    // dump template spec
    if args.dump_template_docs {
        println!("# Template Documentation\n");
        for template in &settings.template_spec {
            println!("{:?}\n", markdown::template_description(&template, 1));
        }
        process::exit(0);
    }

    if let Some(path) = args.texvccheck_path {
        settings.tex_checker = Some(CachedTexChecker::new(&path, 10_000));
    } else {
        eprintln!("Warning: no texvccheck path, won't perform checks!");
    }

    let mut root = if let Some(path) = args.input_file {
        let file = fs::File::open(&path)?;
        serde_json::from_reader(&file)
    } else {
        serde_json::from_reader(io::stdin())
    }
    .expect("Error reading input:");

    root = normalize(root, &settings).expect("Input normalization error:");

    let mut rules = get_rules();
    let mut lints = vec![];

    for mut rule in &mut rules {
        rule.run(&root, &settings, &mut vec![])
            .expect("error while checking rule:");
        lints.append(&mut rule.lints().clone())
    }

    for lint in &lints {
        eprintln!("{}", lint);
        eprintln!("Examples:");
        let examples = get_examples(&rules, lint.kind);
        for example in examples {
            eprintln!("{}", example);
        }
    }

    println!(
        "{}",
        &serde_json::to_string(&lints).expect("could not serialize lints:")
    );
    Ok(())
}
