use std::io;
use std::process;

use clap::{Arg, Command};
use env_logger::Env;
use log::error;
use mdbook_mermaid_mmdr::MermaidPreprocessor;
use mdbook_preprocessor::errors::Error;
use mdbook_preprocessor::Preprocessor;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let matches = Command::new("mdbook-mermaid-mmdr")
        .about("A mdbook preprocessor that renders mermaid diagrams to SVG")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
        .get_matches();

    let preprocessor = MermaidPreprocessor::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        error!("{}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = mdbook_preprocessor::parse_input(io::stdin())?;

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &clap::ArgMatches) {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");
    let supported = pre.supports_renderer(renderer);

    if let Ok(true) = supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
